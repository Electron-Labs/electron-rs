// Copyright © 2022, Electron Labs

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// pub mod near;

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct BigInteger256 {
    val: [u64; 4],
}

impl BigInteger256 {
    pub fn new(src: [u64; 4]) -> Self {
        BigInteger256 { val: src }
    }
}

impl From<BigInteger256> for ark_ff::BigInteger256 {
    fn from(src: BigInteger256) -> ark_ff::BigInteger256 {
        ark_ff::BigInteger256::new(src.val)
    }
}

impl From<ark_ff::BigInteger256> for BigInteger256 {
    fn from(src: ark_ff::BigInteger256) -> BigInteger256 {
        BigInteger256::new(src.0)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
struct Fr {
    c0: BigInteger256,
}

impl Fr {
    pub fn new(src: BigInteger256) -> Self {
        Fr { c0: src }
    }
}

impl From<Fr> for ark_bn254::Fr {
    fn from(src: Fr) -> ark_bn254::Fr {
        ark_bn254::Fr::new(src.c0.into())
    }
}

impl From<ark_bn254::Fr> for Fr {
    fn from(src: ark_bn254::Fr) -> Fr {
        Fr::new(src.0.into())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct Fq {
    c0: BigInteger256,
}

impl Fq {
    pub fn new(src: BigInteger256) -> Self {
        Fq { c0: src }
    }
}

impl From<Fq> for ark_bn254::Fq {
    fn from(src: Fq) -> ark_bn254::Fq {
        ark_bn254::Fq::new(src.c0.into())
    }
}

impl From<ark_bn254::Fq> for Fq {
    fn from(src: ark_bn254::Fq) -> Fq {
        Fq::new(src.0.into())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct Fq2 {
    c0: BigInteger256,
    c1: BigInteger256,
}

impl Fq2 {
    pub fn new(c0_: BigInteger256, c1_: BigInteger256) -> Self {
        Fq2 { c0: c0_, c1: c1_ }
    }
}

impl From<Fq2> for ark_bn254::Fq2 {
    fn from(src: Fq2) -> ark_bn254::Fq2 {
        let c0: ark_ff::BigInteger256 = src.c0.into();
        let c1: ark_ff::BigInteger256 = src.c1.into();
        ark_bn254::Fq2::new(ark_ff::Fp256::new(c0), ark_ff::Fp256::new(c1))
    }
}

impl From<ark_bn254::Fq2> for Fq2 {
    fn from(src: ark_bn254::Fq2) -> Fq2 {
        Fq2::new(src.c0.0.into(), src.c1.0.into())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct Fq6 {
    c0: Fq2,
    c1: Fq2,
    c2: Fq2,
}

impl Fq6 {
    pub fn new(c0_: Fq2, c1_: Fq2, c2_: Fq2) -> Self {
        Fq6 {
            c0: c0_,
            c1: c1_,
            c2: c2_,
        }
    }
}

impl From<Fq6> for ark_bn254::Fq6 {
    fn from(src: Fq6) -> ark_bn254::Fq6 {
        let c0: ark_bn254::Fq2 = src.c0.into();
        let c1: ark_bn254::Fq2 = src.c1.into();
        let c2: ark_bn254::Fq2 = src.c2.into();
        ark_bn254::Fq6::new(c0, c1, c2)
    }
}

impl From<ark_bn254::Fq6> for Fq6 {
    fn from(src: ark_bn254::Fq6) -> Fq6 {
        let c0: ark_bn254::Fq2 = src.c0;
        let c1: ark_bn254::Fq2 = src.c1;
        let c2: ark_bn254::Fq2 = src.c2;
        Fq6::new(c0.into(), c1.into(), c2.into())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct Fq12 {
    c0: Fq6,
    c1: Fq6,
}

impl Fq12 {
    pub fn new(c0_: Fq6, c1_: Fq6) -> Self {
        Fq12 { c0: c0_, c1: c1_ }
    }
}

impl From<Fq12> for ark_bn254::Fq12 {
    fn from(src: Fq12) -> ark_bn254::Fq12 {
        let c0: ark_bn254::Fq6 = src.c0.into();
        let c1: ark_bn254::Fq6 = src.c1.into();
        ark_bn254::Fq12::new(c0, c1)
    }
}

impl From<ark_bn254::Fq12> for Fq12 {
    fn from(src: ark_bn254::Fq12) -> Fq12 {
        let c0: ark_bn254::Fq6 = src.c0;
        let c1: ark_bn254::Fq6 = src.c1;
        Fq12::new(c0.into(), c1.into())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct G1Affine {
    x: BigInteger256,
    y: BigInteger256,
    infinity: bool,
}

impl G1Affine {
    pub fn new(x_: BigInteger256, y_: BigInteger256, infinity_: bool) -> Self {
        G1Affine {
            x: x_,
            y: y_,
            infinity: infinity_,
        }
    }
}

impl From<G1Affine> for ark_bn254::G1Affine {
    fn from(src: G1Affine) -> ark_bn254::G1Affine {
        let x: ark_ff::BigInteger256 = src.x.into();
        let y: ark_ff::BigInteger256 = src.y.into();
        ark_bn254::G1Affine::new(ark_ff::Fp256::new(x), ark_ff::Fp256::new(y), src.infinity)
    }
}

impl From<ark_bn254::G1Affine> for G1Affine {
    fn from(src: ark_bn254::G1Affine) -> G1Affine {
        let x: ark_ff::BigInteger256 = src.x.0;
        let y: ark_ff::BigInteger256 = src.y.0;
        G1Affine::new(x.into(), y.into(), src.infinity)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct G2Affine {
    x: Fq2,
    y: Fq2,
    infinity: bool,
}

impl G2Affine {
    pub fn new(x_: Fq2, y_: Fq2, infinity_: bool) -> Self {
        G2Affine {
            x: x_,
            y: y_,
            infinity: infinity_,
        }
    }
}

impl From<ark_bn254::G2Affine> for G2Affine {
    fn from(src: ark_bn254::G2Affine) -> G2Affine {
        let x: ark_bn254::Fq2 = src.x;
        let y: ark_bn254::Fq2 = src.y;
        G2Affine::new(x.into(), y.into(), src.infinity)
    }
}

impl From<G2Affine> for ark_bn254::G2Affine {
    fn from(src: G2Affine) -> ark_bn254::G2Affine {
        let x: ark_bn254::Fq2 = src.x.into();
        let y: ark_bn254::Fq2 = src.y.into();
        ark_bn254::G2Affine::new(x, y, src.infinity)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct G2Prepared {
    ell_coeffs: Vec<(Fq2, Fq2, Fq2)>,
    infinity: bool,
}

impl G2Prepared {
    pub fn new(ell_coeffs_: Vec<(Fq2, Fq2, Fq2)>, inf: bool) -> Self {
        G2Prepared {
            ell_coeffs: ell_coeffs_,
            infinity: inf,
        }
    }
}

impl From<ark_ec::bn::G2Prepared<ark_bn254::Parameters>> for G2Prepared {
    fn from(src: ark_ec::bn::G2Prepared<ark_bn254::Parameters>) -> G2Prepared {
        let ark_ell_coeffs = src
            .ell_coeffs
            .into_iter()
            .map(|elem| (elem.0, elem.1, elem.2));
        let ell_coeffs: Vec<(Fq2, Fq2, Fq2)> = ark_ell_coeffs
            .map(|elem| (elem.0.into(), elem.1.into(), elem.2.into()))
            .collect();
        G2Prepared::new(ell_coeffs, src.infinity)
    }
}

impl From<G2Prepared> for ark_ec::bn::G2Prepared<ark_bn254::Parameters> {
    fn from(src: G2Prepared) -> ark_ec::bn::G2Prepared<ark_bn254::Parameters> {
        let ark_ell_coeffs = src
            .ell_coeffs
            .into_iter()
            .map(|elem| (elem.0.into(), elem.1.into(), elem.2.into()));
        ark_ec::bn::G2Prepared {
            ell_coeffs: ark_ell_coeffs
                .map(|elem| (elem.0, elem.1, elem.2))
                .collect(),
            infinity: src.infinity,
        }
    }
}

fn fq_from_str(s: String) -> ark_bn254::Fq {
    ark_bn254::Fq::from_str(&s).unwrap()
}

pub fn fr_from_str(s: String) -> ark_bn254::Fr {
    ark_bn254::Fr::from_str(&s).unwrap()
}

fn g1_from_str(g1: &[String]) -> ark_bn254::G1Affine {
    let x = fq_from_str(g1[0].clone());
    let y = fq_from_str(g1[1].clone());
    let z = fq_from_str(g1[2].clone());
    ark_bn254::G1Affine::from(ark_bn254::G1Projective::new(x, y, z))
}

fn g2_from_str(g2: &[Vec<String>]) -> ark_bn254::G2Affine {
    let c0 = fq_from_str(g2[0][0].clone());
    let c1 = fq_from_str(g2[0][1].clone());
    let x = ark_bn254::Fq2::new(c0, c1);

    let c0 = fq_from_str(g2[1][0].clone());
    let c1 = fq_from_str(g2[1][1].clone());
    let y = ark_bn254::Fq2::new(c0, c1);

    let c0 = fq_from_str(g2[2][0].clone());
    let c1 = fq_from_str(g2[2][1].clone());
    let z = ark_bn254::Fq2::new(c0, c1);

    ark_bn254::G2Affine::from(ark_bn254::G2Projective::new(x, y, z))
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct VerifyingKey {
    alpha_g1: G1Affine,
    beta_g2: G2Affine,
    gamma_g2: G2Affine,
    delta_g2: G2Affine,
    gamma_abc_g1: Vec<G1Affine>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct VerifyingKeyPart1 {
    alpha_g1: G1Affine,
    beta_g2: G2Affine,
    gamma_g2: G2Affine,
    delta_g2: G2Affine,
}

impl From<VerifyingKey> for ark_groth16::VerifyingKey<ark_bn254::Bn254> {
    fn from(src: VerifyingKey) -> ark_groth16::VerifyingKey<ark_bn254::Bn254> {
        ark_groth16::VerifyingKey {
            alpha_g1: src.alpha_g1.into(),
            beta_g2: src.beta_g2.into(),
            gamma_g2: src.gamma_g2.into(),
            delta_g2: src.delta_g2.into(),
            gamma_abc_g1: src
                .gamma_abc_g1
                .into_iter()
                .map(|elem| elem.into())
                .collect(),
        }
    }
}

impl From<ark_groth16::VerifyingKey<ark_bn254::Bn254>> for VerifyingKey {
    fn from(src: ark_groth16::VerifyingKey<ark_bn254::Bn254>) -> VerifyingKey {
        VerifyingKey {
            alpha_g1: src.alpha_g1.into(),
            beta_g2: src.beta_g2.into(),
            gamma_g2: src.gamma_g2.into(),
            delta_g2: src.delta_g2.into(),
            gamma_abc_g1: src
                .gamma_abc_g1
                .into_iter()
                .map(|elem| elem.into())
                .collect(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct PreparedVerifyingKey {
    vk: VerifyingKey,
    alpha_g1_beta_g2: Fq12,
    gamma_g2_neg_pc: G2Prepared,
    delta_g2_neg_pc: G2Prepared,
}

impl From<PreparedVerifyingKey> for ark_groth16::PreparedVerifyingKey<ark_bn254::Bn254> {
    fn from(src: PreparedVerifyingKey) -> ark_groth16::PreparedVerifyingKey<ark_bn254::Bn254> {
        ark_groth16::PreparedVerifyingKey {
            vk: src.vk.into(),
            alpha_g1_beta_g2: src.alpha_g1_beta_g2.into(),
            gamma_g2_neg_pc: src.gamma_g2_neg_pc.into(),
            delta_g2_neg_pc: src.delta_g2_neg_pc.into(),
        }
    }
}

impl From<ark_groth16::PreparedVerifyingKey<ark_bn254::Bn254>> for PreparedVerifyingKey {
    fn from(src: ark_groth16::PreparedVerifyingKey<ark_bn254::Bn254>) -> PreparedVerifyingKey {
        PreparedVerifyingKey {
            vk: src.vk.into(),
            alpha_g1_beta_g2: src.alpha_g1_beta_g2.into(),
            gamma_g2_neg_pc: src.gamma_g2_neg_pc.into(),
            delta_g2_neg_pc: src.delta_g2_neg_pc.into(),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone, BorshSerialize, BorshDeserialize)]
pub struct VerificationKeyJson {
    protocol: String,
    curve: String,
    #[serde(rename = "nPublic")]
    num_public: u64,
    vk_alpha_1: Vec<String>,
    vk_beta_2: Vec<Vec<String>>,
    vk_gamma_2: Vec<Vec<String>>,
    vk_delta_2: Vec<Vec<String>>,
    vk_alphabeta_12: Vec<Vec<Vec<String>>>,
    #[serde(rename = "IC")]
    ic: Vec<Vec<String>>,
}

impl From<VerificationKeyJson> for ark_groth16::VerifyingKey<ark_bn254::Bn254> {
    fn from(src: VerificationKeyJson) -> Self {
        let alpha_g1_ = g1_from_str(&src.vk_alpha_1);
        let beta_g2_ = g2_from_str(&src.vk_beta_2);
        let gamma_g2_ = g2_from_str(&src.vk_gamma_2);
        let delta_g2_ = g2_from_str(&src.vk_delta_2);

        let gamma_abc_g1_: Vec<ark_bn254::G1Affine> =
            src.ic.iter().map(|x| g1_from_str(x)).collect();

        ark_groth16::VerifyingKey {
            alpha_g1: alpha_g1_,
            beta_g2: beta_g2_,
            gamma_g2: gamma_g2_,
            delta_g2: delta_g2_,
            gamma_abc_g1: gamma_abc_g1_,
        }
    }
}

impl From<VerificationKeyJson> for VerifyingKey {
    fn from(src: VerificationKeyJson) -> Self {
        let alpha_g1_ = g1_from_str(&src.vk_alpha_1);
        let beta_g2_ = g2_from_str(&src.vk_beta_2);
        let gamma_g2_ = g2_from_str(&src.vk_gamma_2);
        let delta_g2_ = g2_from_str(&src.vk_delta_2);

        let gamma_abc_g1_: Vec<G1Affine> =
            src.ic.iter().map(|x| g1_from_str(x).into()).collect();

        VerifyingKey {
            alpha_g1: alpha_g1_.into(),
            beta_g2: beta_g2_.into(),
            gamma_g2: gamma_g2_.into(),
            delta_g2: delta_g2_.into(),
            gamma_abc_g1: gamma_abc_g1_,
        }
    }
}

impl From<VerificationKeyJson> for VerifyingKeyPart1 {
    fn from(src: VerificationKeyJson) -> Self {
        let alpha_g1_ = g1_from_str(&src.vk_alpha_1);
        let beta_g2_ = g2_from_str(&src.vk_beta_2);
        let gamma_g2_ = g2_from_str(&src.vk_gamma_2);
        let delta_g2_ = g2_from_str(&src.vk_delta_2);

        VerifyingKeyPart1 {
            alpha_g1: alpha_g1_.into(),
            beta_g2: beta_g2_.into(),
            gamma_g2: gamma_g2_.into(),
            delta_g2: delta_g2_.into(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
pub struct CircomProofJson {
    pi_a: Vec<String>,
    pi_b: Vec<Vec<String>>,
    pi_c: Vec<String>,
    protocol: String,
    #[serde(default = "String::new")]
    curve: String,
}

impl From<CircomProofJson> for ark_groth16::Proof<ark_bn254::Bn254> {
    fn from(src: CircomProofJson) -> Self {
        ark_groth16::Proof {
            a: g1_from_str(&src.pi_a),
            b: g2_from_str(&src.pi_b),
            c: g1_from_str(&src.pi_c),
        }
    }
}

/// A helper function to parse verification key json into a prepared
/// verifying key.
pub fn get_prepared_verifying_key(vkey: VerifyingKey) -> PreparedVerifyingKey {
    let parse_vkey: ark_groth16::VerifyingKey<ark_bn254::Bn254> = vkey.into();
    ark_groth16::prepare_verifying_key(&parse_vkey).into()
}
