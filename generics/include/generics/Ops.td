//===- Ops.td - generics operation definitions -------------*- tablegen -*-===//
//
// generics: generic_apply.
// Calls a generic function with concrete type substitutions.
// Resolved by GenericSpecializer pass into func.call to specialized version.
// Reference: Rust monomorphization, Swift SIL apply.
//
//===----------------------------------------------------------------------===//

#ifndef GENERICS_OPS
#define GENERICS_OPS

include "cot/CIR/CIROps.td"
include "generics/Types.td"
include "mlir/IR/SymbolInterfaces.td"

//===----------------------------------------------------------------------===//
// GenericApply — call a generic function with type substitutions
//===----------------------------------------------------------------------===//

def CIR_GenericApplyOp : CIR_Op<"generic_apply", [
    DeclareOpInterfaceMethods<SymbolUserOpInterface>
  ]> {
  let summary = "call a generic function with concrete type substitutions";
  let description = [{
    Calls a generic function identified by `callee` symbol, passing `args`
    and a substitution map `subs` that maps type parameter names to concrete
    types. The GenericSpecializer pass resolves this to a `func.call` to a
    monomorphized clone of the generic function.

    Example:
      %r = cir.generic_apply @identity(%x) subs ["T" = i32] : (i32) -> i32

    Reference: Rust monomorphization, Swift SIL apply with substitution map.
  }];
  let arguments = (ins
    FlatSymbolRefAttr:$callee,
    Variadic<AnyType>:$args,
    StrArrayAttr:$sub_keys,
    TypeArrayAttr:$sub_types
  );
  let results = (outs Variadic<AnyType>:$results);
  let hasCustomAssemblyFormat = 1;
}

#endif // GENERICS_OPS
