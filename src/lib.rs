#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements secp256k1.
//!
//! Curve information:
//! * Base field: q =
//!   115792089237316195423570985008687907853269984665640564039457584007908834671663
//! * Scalar field: r =
//!   115792089237316195423570985008687907852837564279074904382605163141518161494337
//! * Curve equation: y^2 = x^3 + 7

#[cfg(feature = "curve")]
mod curves;
#[cfg(any(feature = "scalar_field", feature = "base_field"))]
mod fields;

#[cfg(feature = "curve")]
pub use curves::*;
#[cfg(any(feature = "scalar_field", feature = "base_field"))]
pub use fields::*;
