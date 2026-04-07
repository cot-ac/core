//===- Types.td - unions type definitions ------------------*- tablegen -*-===//
//
// CIR tagged union type: discriminated union with named variants.
// Reference: Rust enum (with payloads), Swift indirect enum.
//
//===----------------------------------------------------------------------===//

#ifndef UNIONS_TYPES
#define UNIONS_TYPES

include "cot/CIR/CIRTypes.td"

def CIR_TaggedUnionType : CIR_Type<"TaggedUnion", "tagged_union"> {
  let summary = "discriminated union with named typed variants";
  let description = [{
    A tagged union with an i8 discriminator tag and variant payloads.
    `!cir.tagged_union<"Shape", "Circle": f64, "Rect": !cir.struct<...>>`
    Lowers to `!llvm.struct<(i8, [max_payload_bytes x i8])>`.
    Reference: Rust enum with data, Swift indirect enum.
  }];
  let parameters = (ins
    StringRefParameter<"union name">:$name,
    ArrayRefParameter<"::mlir::StringAttr", "variant names">:$variantNames,
    ArrayRefParameter<"::mlir::Type", "variant types">:$variantTypes
  );
  let hasCustomAssemblyFormat = 1;
  let extraClassDeclaration = [{
    /// Get the maximum payload size in bits across all variants.
    unsigned getMaxPayloadBitWidth() const;

    /// Get the index of a variant by name. Returns -1 if not found.
    int64_t getVariantIndex(llvm::StringRef name) const;

    /// Get the type of a variant by name.
    ::mlir::Type getVariantType(llvm::StringRef name) const;
  }];
}

#endif // UNIONS_TYPES
