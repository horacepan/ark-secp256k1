use ark_ec::{
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
};
//use ark_ff::fields::{MontFp, Zero};
use ark_ff::MontFp;

use crate::{fq::Fq, fr::Fr};

//#[cfg(test)]
//mod tests;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Secp256k1Parameters;

impl ModelParameters for Secp256k1Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    }

pub type Affine = GroupAffine<Secp256k1Parameters>;
pub type Projective = GroupProjective<Secp256k1Parameters>;

impl SWModelParameters for Secp256k1Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = MontFp!(Fq, "0");

    /// COEFF_B = 7
    const COEFF_B: Fq = MontFp!(Fq, "7");

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = 1
    const COFACTOR_INV: Fr = MontFp!(Fr, "1");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G_GENERATOR_X, G_GENERATOR_Y);
}

/// G_GENERATOR_X = -1
pub const G_GENERATOR_X: Fq = MontFp!(Fq, "-1");

/// G_GENERATOR_Y = 2
pub const G_GENERATOR_Y: Fq = MontFp!(Fq, "2");
