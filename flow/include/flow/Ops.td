//===- Ops.td - cot-flow operation definitions ---------------*- tablegen -*-===//
//
// cot-flow: br, condbr, switch, trap — all terminators.
// Reference: LLVM IR branch ops, MLIR ControlFlowInterfaces.
//
//===----------------------------------------------------------------------===//

#ifndef FLOW_OPS
#define FLOW_OPS

include "cot/CIR/CIROps.td"
include "cot/CIR/CIROpInterfaces.td"

//===----------------------------------------------------------------------===//
// Unconditional Branch
//===----------------------------------------------------------------------===//

def CIR_BrOp : CIR_Op<"br", [
    Terminator,
    DeclareOpInterfaceMethods<BranchOpInterface>,
    Pure
  ]> {
  let summary = "unconditional branch to a block";
  let description = [{
    Transfers control to `dest`, optionally passing operands.
    Reference: LLVM br, Go fallthrough.
  }];
  let arguments = (ins Variadic<AnyType>:$destOperands);
  let successors = (successor AnySuccessor:$dest);
  let assemblyFormat = [{
    $dest (`(` $destOperands^ `:` type($destOperands) `)`)? attr-dict
  }];
}

//===----------------------------------------------------------------------===//
// Conditional Branch
//===----------------------------------------------------------------------===//

def CIR_CondBrOp : CIR_Op<"condbr", [
    Terminator,
    AttrSizedOperandSegments,
    DeclareOpInterfaceMethods<BranchOpInterface>,
    DeclareOpInterfaceMethods<CIR_ComptimeEvaluable, ["comptimeEval", "hasComptimeSideEffects"]>,
    Pure
  ]> {
  let summary = "conditional branch on i1 condition";
  let description = [{
    Branches to `trueDest` if `condition` is true, else `falseDest`.
    Each branch may pass operands to its destination.
    Reference: LLVM cond_br, Zig branch.
  }];
  let arguments = (ins I1:$condition,
                       Variadic<AnyType>:$trueDestOperands,
                       Variadic<AnyType>:$falseDestOperands);
  let successors = (successor AnySuccessor:$trueDest,
                              AnySuccessor:$falseDest);
  let assemblyFormat = [{
    $condition `,`
    $trueDest (`(` $trueDestOperands^ `:` type($trueDestOperands) `)`)? `,`
    $falseDest (`(` $falseDestOperands^ `:` type($falseDestOperands) `)`)?
    attr-dict
  }];
}

//===----------------------------------------------------------------------===//
// Switch
//===----------------------------------------------------------------------===//

def CIR_SwitchOp : CIR_Op<"switch", [
    Terminator,
    DeclareOpInterfaceMethods<BranchOpInterface>,
    Pure
  ]> {
  let summary = "multi-way branch on integer value";
  let description = [{
    Branches to the case destination matching `value`, or to `defaultDest`
    if no case matches. Case values are i64 constants.
    Reference: LLVM switch, C switch statement.
  }];
  let arguments = (ins AnyInteger:$value,
                       DenseI64ArrayAttr:$case_values);
  let successors = (successor AnySuccessor:$defaultDest,
                              VariadicSuccessor<AnySuccessor>:$caseDests);
  let hasCustomAssemblyFormat = 1;
  let hasVerifier = 1;
}

//===----------------------------------------------------------------------===//
// Trap
//===----------------------------------------------------------------------===//

def CIR_TrapOp : CIR_Op<"trap", [Terminator]> {
  let summary = "abort execution — unreachable code";
  let description = [{
    Emits a trap instruction followed by unreachable. Used for
    unreachable code paths (e.g., after exhaustive switch).
    Reference: LLVM trap + unreachable, Zig @panic.
  }];
  let assemblyFormat = "attr-dict";
}

#endif // FLOW_OPS
