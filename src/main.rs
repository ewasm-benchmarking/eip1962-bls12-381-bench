use eip1962::engines::bls12_381::*;
use eip1962::weierstrass::Group;
use eip1962::pairings::PairingEngine;
use eip1962::traits::{FieldElement,ZeroAndOne};
use eip1962::extension_towers::fp12_as_2_over3_over_2::{Fp12};

fn wasmsnark_test_case() {
    let gt_one = Fp12::one(&BLS12_381_EXTENSION_12_FIELD);

    let mut p = BLS12_381_G1_GENERATOR.clone().mul(vec![27]);
    let mut q = BLS12_381_G2_GENERATOR.clone().mul(vec![37]);
    p.normalize();
    q.normalize();

    let mut p2 = BLS12_381_G1_GENERATOR.clone().mul(vec![999]);
    p2.negate();
    let mut q2 = BLS12_381_G2_GENERATOR.clone();
    p2.normalize();
    q2.normalize();

    let res = BLS12_381_PAIRING_ENGINE.pair(&[p.clone(), p2.clone()], &[q.clone(), q2.clone()]).unwrap();

    assert!(res == gt_one);
}

fn three_point_pairing() {
    let p = BLS12_381_G1_GENERATOR.clone();
    let q = BLS12_381_G2_GENERATOR.clone();

    let mut p2 = p.mul(vec![12345678]);
    p2.normalize();

    let mut q2 = q.mul(vec![12345678]);
    q2.normalize();

    let ans1 = BLS12_381_PAIRING_ENGINE.pair(&[p.clone()], &[q2]).unwrap();
    let ans2 = BLS12_381_PAIRING_ENGINE.pair(&[p2], &[q.clone()]).unwrap();
    let ans3 = BLS12_381_PAIRING_ENGINE.pair(&[p], &[q]).unwrap();
    let ans3 = ans3.pow(&vec![12345678]);

    assert!(ans1 == ans2);
    assert!(ans1 == ans3);
}

fn main() {
    // three_point_pairing();
    websnark_test_case();
}
