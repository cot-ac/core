//===- Types.td - generics type definitions ----------------*- tablegen -*-===//
//
// CIR type parameter: represents a generic type variable in unspecialized code.
// After GenericSpecializer runs, no type_param types remain in concrete functions.
// Reference: Swift SIL archetype types, Rust monomorphization.
//
//===----------------------------------------------------------------------===//

#ifndef GENERICS_TYPES
#define GENERICS_TYPES

include "cot/CIR/CIRTypes.td"

def CIR_TypeParamType : CIR_Type<"TypeParam", "type_param"> {
  let summary = "generic type parameter (resolved by specialization)";
  let description = [{
    A named type parameter representing a generic type variable.
    `!cir.type_param<"T">` appears in generic function signatures and bodies.
    The GenericSpecializer pass clones generic functions for each concrete
    type instantiation, replacing type_param with the concrete type.
    Falls back to `!llvm.ptr` if it reaches lowering unresolved.
    Reference: Swift SIL archetype, Rust type parameter.
  }];
  let parameters = (ins StringRefParameter<"parameter name">:$name);
  let assemblyFormat = "`<` $name `>`";
}

#endif // GENERICS_TYPES
