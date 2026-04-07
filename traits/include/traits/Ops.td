//===- Ops.td - traits operation definitions ---------------*- tablegen -*-===//
//
// Trait/protocol ops: witness tables, static trait dispatch, dynamic
// dispatch through existential containers.
// Reference: Swift SIL witness tables + existential ops.
//
//===----------------------------------------------------------------------===//

#ifndef TRAITS_OPS
#define TRAITS_OPS

include "cot/CIR/CIROps.td"
include "traits/Types.td"
include "mlir/IR/SymbolInterfaces.td"

//===----------------------------------------------------------------------===//
// WitnessTable — compile-time protocol conformance declaration
//===----------------------------------------------------------------------===//

def CIR_WitnessTableOp : CIR_Op<"witness_table", [
    Symbol
  ]> {
  let summary = "protocol witness table declaration";
  let description = [{
    Declares a witness table mapping protocol method names to concrete
    function implementations. Used by GenericSpecializer to resolve
    trait_call ops at compile time, and by WitnessTableGenerator to
    emit LLVM global constant structs for runtime dispatch.

    Example:
      cir.witness_table @Summable_Point ["sum" = @Point_sum]
  }];
  let arguments = (ins
    SymbolNameAttr:$sym_name,
    StrAttr:$protocol,
    StrAttr:$conforming_type,
    StrArrayAttr:$method_names,
    SymbolRefArrayAttr:$method_impls
  );
  let hasCustomAssemblyFormat = 1;
}

//===----------------------------------------------------------------------===//
// TraitCall — static trait dispatch (resolved by GenericSpecializer)
//===----------------------------------------------------------------------===//

def CIR_TraitCallOp : CIR_Op<"trait_call", [
    DeclareOpInterfaceMethods<SymbolUserOpInterface>
  ]> {
  let summary = "call a protocol method on a concrete type (static dispatch)";
  let description = [{
    Calls a protocol method by name, dispatching through the witness table.
    GenericSpecializer resolves this to a direct func.call by looking up
    the witness table entry for the concrete type.

    Example:
      %r = cir.trait_call @Summable::sum(%val) : (!cir.type_param<"T">) -> i32
  }];
  let arguments = (ins
    FlatSymbolRefAttr:$protocol,
    StrAttr:$method,
    Variadic<AnyType>:$args
  );
  let results = (outs Variadic<AnyType>:$results);
  let hasCustomAssemblyFormat = 1;
}

//===----------------------------------------------------------------------===//
// WitnessMethod — dynamic trait dispatch (runtime vtable lookup)
//===----------------------------------------------------------------------===//

def CIR_WitnessMethodOp : CIR_Op<"witness_method", [Pure]> {
  let summary = "load a method pointer from a protocol witness table";
  let description = [{
    Loads the function pointer for a protocol method from a Protocol
    Witness Table pointer at runtime. Used for dynamic dispatch through
    existential containers.

    method_index specifies the slot in the PWT array. If not provided,
    a resolution pass must set it before lowering.

    Example:
      %fn = cir.witness_method %pwt, "sum"[0] : (!cir.ptr) -> !cir.ptr
  }];
  let arguments = (ins
    AnyType:$pwt,
    StrAttr:$method,
    OptionalAttr<I64Attr>:$method_index
  );
  let results = (outs AnyType:$result);
  let hasCustomAssemblyFormat = 1;
}

//===----------------------------------------------------------------------===//
// Existential container ops
//===----------------------------------------------------------------------===//

def CIR_InitExistentialOp : CIR_Op<"init_existential", []> {
  let summary = "package a concrete value into an existential container";
  let description = [{
    Stores a concrete value plus its VWT and PWT pointers into an
    existential container. The container is allocated by the caller
    (typically via alloca).

    Example:
      cir.init_existential %container, %value, %vwt, %pwt
        : !cir.existential<"Summable">, i32, !cir.ptr, !cir.ptr
  }];
  let arguments = (ins
    AnyType:$container,
    AnyType:$value,
    AnyType:$vwt,
    AnyType:$pwt
  );
  let assemblyFormat = [{
    $container `,` $value `,` $vwt `,` $pwt attr-dict
    `:` type($container) `,` type($value) `,` type($vwt) `,` type($pwt)
  }];
}

def CIR_OpenExistentialOp : CIR_Op<"open_existential", [Pure]> {
  let summary = "extract buffer, VWT, and PWT from an existential container";
  let description = [{
    Opens an existential container to access its components: the value
    buffer pointer, the VWT pointer, and the PWT pointer.

    Example:
      %buf, %vwt, %pwt = cir.open_existential %container
        : !cir.existential<"Summable"> -> (!cir.ptr, !cir.ptr, !cir.ptr)
  }];
  let arguments = (ins AnyType:$container);
  let results = (outs AnyType:$buffer, AnyType:$vwt, AnyType:$pwt);
  let hasCustomAssemblyFormat = 1;
}

def CIR_DeinitExistentialOp : CIR_Op<"deinit_existential", []> {
  let summary = "clean up an existential container";
  let description = [{
    Nulls out the metadata slots of an existential container.
    The value itself must be destroyed separately via VWT.destroy.

    Example:
      cir.deinit_existential %container : !cir.existential<"Summable">
  }];
  let arguments = (ins AnyType:$container);
  let assemblyFormat = "$container attr-dict `:` type($container)";
}

#endif // TRAITS_OPS
