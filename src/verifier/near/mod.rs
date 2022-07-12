// Copyright © 2022, Electron Labs

use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::Deserialize;
use serde_json_wasm;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("Failed to parse circom {0} json")]
    ParseError(String),
}

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

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
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
#[derive(Deserialize, Clone)]
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(crate = "near_sdk::serde")]
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

fn parse_circom_proof(proof: String) -> Result<CircomProofJson> {
    let proof = serde_json_wasm::from_str(&proof)
        .map_err(|_| VerifierError::ParseError("proof".to_string()))?;
    Ok(proof)
}

fn parse_public_inputs(inputs: String) -> Result<Vec<String>> {
    let pub_inputs: Vec<String> = serde_json_wasm::from_str(&inputs)
        .map_err(|_| VerifierError::ParseError("public inputs".to_string()))?;
    Ok(pub_inputs)
}

/// A helper function to parse raw verification key json returned by circom.
///
/// # Errors
/// VerifierError::VkeyParseError
///
/// This function will return an error if it fails to parse the verification
/// key json file returned by circom.
pub fn parse_verification_key(vkey_str: String) -> Result<VerificationKeyJson> {
    let vkey = serde_json_wasm::from_str(&vkey_str)
        .map_err(|_| VerifierError::ParseError("verification key".to_string()))?;
    Ok(vkey)
}

/// A helper function to parse verification key json into a prepared
/// verifying key.
pub fn get_prepared_verifying_key(vkey: VerificationKeyJson) -> PreparedVerifyingKey {
    let parse_vkey: ark_groth16::VerifyingKey<ark_bn254::Bn254> = vkey.into();
    ark_groth16::prepare_verifying_key(&parse_vkey).into()
}

/// A helper function to verify proof
pub fn verify_proof(
    pvk: PreparedVerifyingKey,
    proof_str: String,
    pub_inputs_str: String,
) -> Result<bool> {
    let proof = parse_circom_proof(proof_str)?;
    let pub_inputs = parse_public_inputs(pub_inputs_str)?;
    let ark_pub_inputs: Vec<ark_bn254::Fr> = pub_inputs.into_iter().map(fr_from_str).collect();

    // TODO: Convert this to a proper error type of Bolt-rs
    let res = ark_groth16::verify_proof(&pvk.into(), &proof.into(), &ark_pub_inputs[..]).unwrap();

    Ok(res)
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
            "nPublic": 21,
            "vk_alpha_1": [
             "20491192805390485299153009773594534940189261866228447918068658471970481763042",
             "9383485363053290200918347156157836566562967994039712273449902621266178545958",
             "1"
            ],
            "vk_beta_2": [
             [
              "6375614351688725206403948262868962793625744043794305715222011528459656738731",
              "4252822878758300859123897981450591353533073413197771768651442665752259397132"
             ],
             [
              "10505242626370262277552901082094356697409835680220590971873171140371331206856",
              "21847035105528745403288232691147584728191162732299865338377159692350059136679"
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
              "166438788818422684353143109466712365495487529761282054253940311767202847529",
              "14821889692288092546390398853883577003395705920427691037003877337111307008319"
             ],
             [
              "5211044291848451570308359449705497730711843248959818951644537468318735026319",
              "3349759874590271776701023934351541831283252450166481144436728710799565826635"
             ],
             [
              "1",
              "0"
             ]
            ],
            "vk_alphabeta_12": [
             [
              [
               "2029413683389138792403550203267699914886160938906632433982220835551125967885",
               "21072700047562757817161031222997517981543347628379360635925549008442030252106"
              ],
              [
               "5940354580057074848093997050200682056184807770593307860589430076672439820312",
               "12156638873931618554171829126792193045421052652279363021382169897324752428276"
              ],
              [
               "7898200236362823042373859371574133993780991612861777490112507062703164551277",
               "7074218545237549455313236346927434013100842096812539264420499035217050630853"
              ]
             ],
             [
              [
               "7077479683546002997211712695946002074877511277312570035766170199895071832130",
               "10093483419865920389913245021038182291233451549023025229112148274109565435465"
              ],
              [
               "4595479056700221319381530156280926371456704509942304414423590385166031118820",
               "19831328484489333784475432780421641293929726139240675179672856274388269393268"
              ],
              [
               "11934129596455521040620786944827826205713621633706285934057045369193958244500",
               "8037395052364110730298837004334506829870972346962140206007064471173334027475"
              ]
             ]
            ],
            "IC": [
             [
              "19975645442203377055504350944199411205645925605842881710313661501103970826593",
              "17515161622283010384423259590087060433422690594791060414171309961412819784969",
              "1"
             ],
             [
              "8314529012362679498714409542216060373647165806213078732764739247682086265767",
              "121366207716244222195924313927761544312158108247873731042786280646943184074",
              "1"
             ],
             [
              "16709720837782968526180617884167855231344603866174025119200385206304701258678",
              "3147822512060247213265367088074297137791420360497197470911250310113275037763",
              "1"
             ],
             [
              "14216723210244410575876418879665374598747581482663712212010511617392597830954",
              "15811996758528967218865995673654714048570588460636125402018277656651434631576",
              "1"
             ],
             [
              "7348238908009886871059992732128931157271697524606274111411455960455037416413",
              "14001472805890407823397893627240743988837305207489952388063413323698861707624",
              "1"
             ],
             [
              "2138882192497635891459717929673559440104769163700828386965661447497938982721",
              "5186793583243682306353927402481196491547812815293709454908025411581465445004",
              "1"
             ],
             [
              "2116764452247307873087707246637130330345204236852642632713114592476993977670",
              "14896161713831569254989869822450928542555444355351318861266435690413316845347",
              "1"
             ],
             [
              "16392430006950202355682918247811738427580100868571691215288876389925500647279",
              "19437084047439114680241004405825353549565621104782399561893962443338240135858",
              "1"
             ],
             [
              "16963065381115919041780779888616737843143206987161162977928288398707149790618",
              "9087066945988971374305861013885116715721320414719802148300649773920118102481",
              "1"
             ],
             [
              "13714673228950478504452201663230221577251226934030004828193127473877480610295",
              "9332072320101623120415187992550525752876274301602491265535702933221101004380",
              "1"
             ],
             [
              "1064045990922553586834518447367936820175319540784875187573912133883165188670",
              "18287981330912970040426745735838860702735392209815444404076135459948276202848",
              "1"
             ],
             [
              "9210826867500141415001909980706988517816622370128886786816673451224513701503",
              "3651094788905360180553273507287364045940819368096000322156684552199804097143",
              "1"
             ],
             [
              "17720362295505313322759315353391656693108343058592864160681048989141882794083",
              "10097671657793855671159749436121468469201270375403582850205385628210921488731",
              "1"
             ],
             [
              "9801543874486422221954003660705098546171144064277720948049325854942931758306",
              "20479944074043794678092216875190551894013835948904068657881623722226189539016",
              "1"
             ],
             [
              "5374663040433250412848838440386505484894911153493652424898166227177046711199",
              "13679665179607144765496503536099360866217236185602567461732884358192393872279",
              "1"
             ],
             [
              "1064329530975255434535409396597644022861254752006703233721201637345800440139",
              "5140009461438788926486789050955593582109349287858692508879168080077367120629",
              "1"
             ],
             [
              "15366436033551689602012357199098419434258945123964889817106842055644617190504",
              "898268788386333715715903230667785887632210104432209295828625929694299885006",
              "1"
             ],
             [
              "5625417729666095139456177838606211212046421091440422619829111829213675828978",
              "18455517249670178543137281808225159109856379895586238312217422816116366743603",
              "1"
             ],
             [
              "17537235019815029148949517328224734386526017513684721827218738801833451783210",
              "2342105886191919519714066767578407697780765722350456533494274069027087830216",
              "1"
             ],
             [
              "8512191115799353035296472708809096858085180357544392842547774011355858433041",
              "2541245043439530389724749443817975569327264943016202232800605721736943199048",
              "1"
             ],
             [
              "19224585989189727449965872368330162278522031170641583311558474979239173678715",
              "18166021891232232834725962994255689261693690030629187665379835418854223722023",
              "1"
             ],
             [
              "14017181509831449693830612331037537298674425286306310710534048602053149127774",
              "330831566870832606085453648362982294226755734586757078631724647552023101374",
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
        assert_eq!(vkey.num_public, 21);
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
            "Failed to parse circom verification key json"
        );
    }

    #[test]
    fn test_prepared_verification_key() {
        let vkey_str = get_vkey();
        let vkey = parse_verification_key(vkey_str.to_string()).unwrap();
        let prepared_vkey = get_prepared_verifying_key(vkey);
        let x: BigInteger256 = BigInteger256::new([
            129941079445278231,
            14986904513597369283,
            4385962745611939561,
            498495035870568143,
        ]);
        let y: BigInteger256 = BigInteger256::new([
            3551982070992374558,
            4387704605030068278,
            1260785428361773688,
            452138810654549394,
        ]);
        let g1 = G1Affine::new(x, y, false);

        assert_eq!(g1, prepared_vkey.vk.alpha_g1);
    }

    #[test]
    fn test_parse_public_input() {
        let pub_input_str = r#"[
            "1",
            "277989581668086710587965336712738880284",
            "314891321346369595428838678892844352460"
        ]"#;
        let inputs = parse_public_inputs(pub_input_str.to_string()).unwrap();
        assert_eq!("1", inputs[0]);
        assert_eq!("277989581668086710587965336712738880284", inputs[1]);
        assert_eq!("314891321346369595428838678892844352460", inputs[2]);
    }

    #[test]
    fn test_valid_proof_snarkjs() {
        let proof_str = r#"
        {
            "pi_a": [
              "20198676790799425245595459194274498752473994950719073183074649501711660535595",
              "12758475309915023533579531485441554907458299575042834087971469653289637732346",
              "1"
            ],
            "pi_b": [
              [
                "13742117572560123711123425096963974481037753438772131102525214062174465939468",
                "9217768357543713672348398426848893195759877300475465964741673960918197283129"
              ],
              [
                "13388985823083338129254299703944286332336674476925977438789020739020226493083",
                "13389941977815367065802562753053209214146349395284722106316234427940539426898"
              ],
              [
                "1",
                "0"
              ]
            ],
            "pi_c": [
              "5988936190268741469108357726405145464702633179533876088993318355641592876129",
              "15053058905266236652562457399329328685910831643948235107886315836157181001907",
              "1"
            ],
            "protocol": "groth16",
            "curve": "bn128"
        }
        "#;
        let pub_input_str = r#"
        [
            "1",
            "139034790179591340742761703217010858871",
            "178747724383637324525799708680472596098",
            "249730154399878769526315894913495941533",
            "339453732354324016397146782775657558721",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052"
        ]
        "#;
        let vkey_str = get_vkey();
        let vkey = parse_verification_key(vkey_str.to_string()).unwrap();
        let prepared_vkey = get_prepared_verifying_key(vkey);

        let res = verify_proof(
            prepared_vkey,
            proof_str.to_string(),
            pub_input_str.to_string(),
        );
        assert!(res.unwrap());
    }

    #[test]
    fn test_valid_proof_rapidsnark() {
        let proof_str = r#"
        {
            "pi_a": [
              "20198676790799425245595459194274498752473994950719073183074649501711660535595",
              "12758475309915023533579531485441554907458299575042834087971469653289637732346",
              "1"
            ],
            "pi_b": [
              [
                "13742117572560123711123425096963974481037753438772131102525214062174465939468",
                "9217768357543713672348398426848893195759877300475465964741673960918197283129"
              ],
              [
                "13388985823083338129254299703944286332336674476925977438789020739020226493083",
                "13389941977815367065802562753053209214146349395284722106316234427940539426898"
              ],
              [
                "1",
                "0"
              ]
            ],
            "pi_c": [
              "5988936190268741469108357726405145464702633179533876088993318355641592876129",
              "15053058905266236652562457399329328685910831643948235107886315836157181001907",
              "1"
            ],
            "protocol": "groth16"
        }
        "#;
        let pub_input_str = r#"
        [
            "1",
            "139034790179591340742761703217010858871",
            "178747724383637324525799708680472596098",
            "249730154399878769526315894913495941533",
            "339453732354324016397146782775657558721",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052",
            "208326850591216812292393721318634961999",
            "28902942442541169865286267622270965052"
        ]
        "#;
        let vkey_str = get_vkey();
        let vkey = parse_verification_key(vkey_str.to_string()).unwrap();
        let prepared_vkey = get_prepared_verifying_key(vkey);

        let res = verify_proof(
            prepared_vkey,
            proof_str.to_string(),
            pub_input_str.to_string(),
        );
        assert!(res.unwrap());
    }
}
