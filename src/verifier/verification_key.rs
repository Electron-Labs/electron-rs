// Copyright © 2022, Electron Labs

use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::Deserialize;
use serde_json_wasm;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("Failed to parse verification key json")]
    VkeyParseError(#[from] serde_json_wasm::de::Error),
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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
        ark_bn254::Fq2::new(c0.into(), c1.into())
    }
}

impl From<ark_bn254::Fq2> for Fq2 {
    fn from(src: ark_bn254::Fq2) -> Fq2 {
        Fq2::new(src.c0.0.into(), src.c1.0.into())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
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
        ark_bn254::G1Affine::new(x.into(), y.into(), src.infinity)
    }
}

impl From<ark_bn254::G1Affine> for G1Affine {
    fn from(src: ark_bn254::G1Affine) -> G1Affine {
        let x: ark_ff::BigInteger256 = src.x.into();
        let y: ark_ff::BigInteger256 = src.y.into();
        G1Affine::new(x.into(), y.into(), src.infinity)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct VerifyingKey {
    alpha_g1: G1Affine,
    beta_g2: G2Affine,
    gamma_g2: G2Affine,
    delta_g2: G2Affine,
    gamma_abc_g1: Vec<G1Affine>,
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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
#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde")]
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

/// A helper function to parse raw verification key json returned by circom.
///
/// # Errors
/// VerifierError::VkeyParseError
///
/// This function will return an error if it fails to parse the verification
/// key json file returned by circom.
pub fn parse_verification_key(vkey_str: String) -> Result<VerificationKeyJson> {
    let vkey = serde_json_wasm::from_str(&vkey_str).map_err(VerifierError::VkeyParseError)?;
    Ok(vkey)
}

/// A helper function to parse verification key json into a prepared
/// verifying key.
pub fn get_prepared_verifying_key(vkey: VerificationKeyJson) -> PreparedVerifyingKey {
    let parse_vkey: ark_groth16::VerifyingKey<ark_bn254::Bn254> = vkey.into();
    ark_groth16::prepare_verifying_key(&parse_vkey).into()
}

fn fq_from_str(s: String) -> ark_bn254::Fq {
    ark_bn254::Fq::from_str(&s).unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_vkey() -> &'static str {
        r#"
        {
            "protocol": "groth16",
            "curve": "bn128",
            "nPublic": 1,
            "vk_alpha_1": [
             "8604667279420059501166553404773392135946736498054200992926926673060716660829",
             "20360062655515720670379654635393394213543483323604306907537674392480557199402",
             "1"
            ],
            "vk_beta_2": [
             [
              "3043345541449451671254450988991753002656660035846876061355339859941832004318",
              "9366036361352379018594831897170702393774275718261521192392224789664885279048"
             ],
             [
              "13013276479422120091160662049955987435710012286605495842546532018893262680987",
              "14570689804927635484337933117252889899638838990887585775285700495326742500608"
             ],
             [
              "1",
              "0"
             ]
            ],
            "vk_gamma_2": [
             [
              "10857046999023057135944570762232829481370756359578518086990519993285655852781",
              "11559732032986387107991004021392285783925812861821192530917403151452391805634"
             ],
             [
              "8495653923123431417604973247489272438418190587263600148770280649306958101930",
              "4082367875863433681332203403145435568316851327593401208105741076214120093531"
             ],
             [
              "1",
              "0"
             ]
            ],
            "vk_delta_2": [
             [
              "2734577848493477101784673168916689189152190109550989513773621988766628013896",
              "20992053824921110769159163698868426355728207712097181822869885973257549986665"
             ],
             [
              "18858728426139214530397886298764629051579512035453478146479294850808428228226",
              "16650869185745772661582147225220884707446064820553793560683789497559329846949"
             ],
             [
              "1",
              "0"
             ]
            ],
            "vk_alphabeta_12": [
             [
              [
               "16403249578719490126072533602691465129046148109548339172692159293394071643386",
               "404083172884130787011880125990917651653105025179849649507378532814812033743"
              ],
              [
               "9304686624381716522435760802435389234743930919257995833040856535135604271150",
               "7297682513079878497741793281328503938637410885893249606756478534362180551835"
              ],
              [
               "17354121827975724085057788554351619925552460622743302973070914719205720740971",
               "17029060856042991949050729552637497721303925875794209439369222736105594947627"
              ]
             ],
             [
              [
               "7351439472145663460303190808494715160984862908266037163980906171905322144196",
               "4899830427462111655211072420148616010374627247339367569243756669854362971692"
              ],
              [
               "11493816073724792379849577567766940080449553642999473348500952038852103043242",
               "20229043536413592615655482855555253411102261009946710916737819944352224248089"
              ],
              [
               "20445551610334345159229738787294472467932316248073152880543490797842284204290",
               "4991801408713450062187668790956752622553932388084176879623832267439560781668"
              ]
             ]
            ],
            "IC": [
             [
              "20510024326636861894856056279186972251820656064299818504132684390781123564002",
              "3794043495370927585051135397901732182692326739063049522454286904701134003013",
              "1"
             ],
             [
              "7791962724153994122113202116325467726962116651195725568779661762583649623632",
              "21733435539045095673745804075891544265305400637072500486664710068860705765791",
              "1"
             ]
            ]
           }
        "#
    }

    #[test]
    fn test_parse_valid_verification_key() {
        let vkey_str = get_vkey();
        let vkey = parse_verification_key(vkey_str.to_string()).unwrap();
        assert_eq!(vkey.protocol, "groth16");
        assert_eq!(vkey.curve, "bn128");
        assert_eq!(vkey.num_public, 1);
    }

    #[test]
    fn test_parse_invalid_verification_key() {
        let vkey_str = r#"
        {
            "protocol": "groth16",
            "curve": "bn128",
            "nPublic": 1,
            "vk_beta_2": [
             [
              "3043345541449451671254450988991753002656660035846876061355339859941832004318",
              "9366036361352379018594831897170702393774275718261521192392224789664885279048"
             ],
             [
              "13013276479422120091160662049955987435710012286605495842546532018893262680987",
              "14570689804927635484337933117252889899638838990887585775285700495326742500608"
             ],
             [
              "1",
              "0"
             ]
            ],
            "vk_gamma_2": [
             [
              "10857046999023057135944570762232829481370756359578518086990519993285655852781",
              "11559732032986387107991004021392285783925812861821192530917403151452391805634"
             ],
             [
              "8495653923123431417604973247489272438418190587263600148770280649306958101930",
              "4082367875863433681332203403145435568316851327593401208105741076214120093531"
             ],
             [
              "1",
              "0"
             ]
            ],
            "vk_delta_2": [
             [
              "2734577848493477101784673168916689189152190109550989513773621988766628013896",
              "20992053824921110769159163698868426355728207712097181822869885973257549986665"
             ],
             [
              "18858728426139214530397886298764629051579512035453478146479294850808428228226",
              "16650869185745772661582147225220884707446064820553793560683789497559329846949"
             ],
             [
              "1",
              "0"
             ]
            ],
            "vk_alphabeta_12": [
             [
              [
               "16403249578719490126072533602691465129046148109548339172692159293394071643386",
               "404083172884130787011880125990917651653105025179849649507378532814812033743"
              ],
              [
               "9304686624381716522435760802435389234743930919257995833040856535135604271150",
               "7297682513079878497741793281328503938637410885893249606756478534362180551835"
              ],
              [
               "17354121827975724085057788554351619925552460622743302973070914719205720740971",
               "17029060856042991949050729552637497721303925875794209439369222736105594947627"
              ]
             ],
             [
              [
               "7351439472145663460303190808494715160984862908266037163980906171905322144196",
               "4899830427462111655211072420148616010374627247339367569243756669854362971692"
              ],
              [
               "11493816073724792379849577567766940080449553642999473348500952038852103043242",
               "20229043536413592615655482855555253411102261009946710916737819944352224248089"
              ],
              [
               "20445551610334345159229738787294472467932316248073152880543490797842284204290",
               "4991801408713450062187668790956752622553932388084176879623832267439560781668"
              ]
             ]
            ],
            "IC": [
             [
              "20510024326636861894856056279186972251820656064299818504132684390781123564002",
              "3794043495370927585051135397901732182692326739063049522454286904701134003013",
              "1"
             ],
             [
              "7791962724153994122113202116325467726962116651195725568779661762583649623632",
              "21733435539045095673745804075891544265305400637072500486664710068860705765791",
              "1"
             ]
            ]
           }
        "#;
        let vkey = parse_verification_key(vkey_str.to_string());
        assert!(vkey.is_err());
        assert_eq!(
            vkey.err().expect("Invalid Vkey").to_string(),
            "Failed to parse verification key json"
        );
    }

    #[test]
    fn test_prepared_verification_key() {
        let vkey_str = get_vkey();
        let vkey = parse_verification_key(vkey_str.to_string()).unwrap();
        let prepared_vkey = get_prepared_verifying_key(vkey);
        let x: BigInteger256 = BigInteger256::new([
            3849113555213797469,
            6739222786987396424,
            12326519530335657568,
            1370802584083018133,
        ]);
        let y: BigInteger256 = BigInteger256::new([
            4512379106493624362,
            17993990849293210707,
            4595186547289003824,
            3243545112665201079,
        ]);
        let g1 = G1Affine::new(x, y, false);

        assert_eq!(g1, prepared_vkey.vk.alpha_g1);
    }
}