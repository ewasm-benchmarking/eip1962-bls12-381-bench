use std::time::{Instant};

use eip1962::engines::bls12_381::*;
use eip1962::weierstrass::Group;
use eip1962::pairings::PairingEngine;
use eip1962::traits::{FieldElement,ZeroAndOne};
use eip1962::extension_towers::fp12_as_2_over3_over_2::{Fp12};

fn bench_two_point_pairing() {
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

    let start = Instant::now();
    let mut res = Fp12::zero(&BLS12_381_EXTENSION_12_FIELD);
    for i in 0..1000 {
        res = BLS12_381_PAIRING_ENGINE.pair(&[p.clone(), p2.clone()], &[q.clone(), q2.clone()]).unwrap();
    }


    let duration = start.elapsed();
    println!("Time elapsed in bench() is: {:?}", duration);

    assert!(res == gt_one);
}

fn main() {
    bench_two_point_pairing();
}
