//===- Types.td - traits type definitions ------------------*- tablegen -*-===//
//
// CIR existential type: a value conforming to a named protocol.
// Lowers to a 3-word struct: [24-byte inline buffer, VWT ptr, PWT ptr].
// Reference: Swift existential containers, Rust dyn Trait fat pointers.
//
//===----------------------------------------------------------------------===//

#ifndef TRAITS_TYPES
#define TRAITS_TYPES

include "cot/CIR/CIRTypes.td"

def CIR_ExistentialType : CIR_Type<"Existential", "existential"> {
  let summary = "type-erased value conforming to a protocol";
  let description = [{
    An existential container holds any value conforming to the named protocol.
    `!cir.existential<"Summable">` can hold any type that implements Summable.
    Layout: 24-byte inline buffer (small values stored directly, large values
    heap-allocated), a pointer to the Value Witness Table (VWT), and a pointer
    to the Protocol Witness Table (PWT).
    Reference: Swift existential containers, Rust dyn Trait.
  }];
  let parameters = (ins StringRefParameter<"protocol name">:$name);
  let assemblyFormat = "`<` $name `>`";
}

#endif // TRAITS_TYPES
