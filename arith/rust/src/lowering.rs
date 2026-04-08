//! # Arith Lowering
//!
//! Lowers CIR arith ops to Cranelift IR instructions:
//! - `add` -> `iadd` / `fadd`
//! - `sub` -> `isub` / `fsub`
//! - `mul` -> `imul` / `fmul`
//! - `div` -> `sdiv` / `udiv` / `fdiv`
//! - `rem` -> `srem` / `urem`
//! - `neg` -> `ineg` / `fneg`
//! - `cmp` -> `icmp`
//! - `cmpf` -> `fcmp`
//! - `select` -> `select`
//! - `bit_and` -> `band`, `bit_or` -> `bor`, `bit_xor` -> `bxor`
//! - `bit_not` -> `bnot`
//! - `shl` -> `ishl`, `shr` -> `sshr` / `ushr`
//! - Integer casts -> `sextend`, `uextend`, `ireduce`
//! - Float casts -> `fcvt_to_sint`, `fcvt_from_sint`, `fpromote`, `fdemote`
//! - Constants -> `iconst`, `f32const`, `f64const`
//!
//! Phase 3 — not yet implemented.

// Lowering will be implemented in Phase 3 when we add the Cranelift backend.
