use eip1962::engines::bls12_381::*;
use eip1962::weierstrass::curve::{CurvePoint};
use eip1962::weierstrass::{CurveOverFpParameters, CurveOverFp2Parameters};
use eip1962::traits::{ZeroAndOne};
use eip1962::extension_towers::fp12_as_2_over3_over_2::{Fp12};
use eip1962::extension_towers::fp2::Fp2;
use eip1962::field::{U384Repr, PrimeField};
use eip1962::fp::Fp;
use eip1962::{decl_fp, decl_fp2};

const SIZE_F1: usize = 6;
const SIZE_P_G1: usize = SIZE_F1 * 3;
const SIZE_P_G2: usize = SIZE_F1 * 6;
const INPUT_SIZE: usize = SIZE_P_G1 + SIZE_P_G2;

fn parse_g1_point(data: &[u64]) -> CurvePoint<CurveOverFpParameters<'static, U384Repr, PrimeField<U384Repr>>> {
    let mut p_x_data = [0u64; SIZE_F1];
    let mut p_y_data = [0u64; SIZE_F1];

    p_x_data.clone_from_slice(&data[0..12]);
    p_y_data.clone_from_slice(&data[12..24]);

    let mut p_x = Fp::from_repr(&BLS12_381_FIELD, U384Repr(p_x_data)).expect("invalid point");
    let mut p_y = Fp::from_repr(&BLS12_381_FIELD, U384Repr(p_y_data)).expect("invalid point");
    CurvePoint::point_from_xy(&BLS12_381_G1_CURVE, p_x, p_y)
}

fn parse_g2_point(data: &[u64]) -> CurvePoint<CurveOverFp2Parameters<'static, U384Repr, PrimeField<U384Repr>>> {
    let mut p_x_data = [0u64; SIZE_F1];
    let mut p_y_data = [0u64; SIZE_F1];

    p_x_data.clone_from_slice(&data[0..12]);
    p_y_data.clone_from_slice(&data[12..24]);

    let mut p_x = Fp2::from_repr(&BLS12_381_FIELD, U384Repr(p_x_data)).expect("invalid point");
    let mut p_y = Fp::from_repr(&BLS12_381_FIELD, U384Repr(p_y_data)).expect("invalid point");
    CurvePoint::point_from_xy(&BLS12_381_G2_CURVE, p_x, p_y)
}

pub fn bench() {
    //let input: [u64; INPUT_SIZE] = [0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64];

    //{{input}}

    /*
    let mut p_x_data = [0u64; SIZE_F1];
    let mut p_y_data = [0u64; SIZE_F1];

    p_x_data.clone_from_slice(&input[0..6]);
    p_y_data.clone_from_slice(&input[6..12]);

    let mut p_x = Fp::from_repr(&BLS12_381_FIELD, U384Repr(p_x_data)).expect("invalid point");
    let mut p_y = Fp::from_repr(&BLS12_381_FIELD, U384Repr(p_y_data)).expect("invalid point");

    let mut p1 = CurvePoint::point_from_xy(&BLS12_381_G1_CURVE, p_x, p_y);
    let mut p2 = CurvePoint::point_from_xy(&BLS12_381_G2_CURVE, p_x, p_y);
    let mut p3 = CurvePoint::point_from_xy(&BLS12_381_G1_CURVE, p_x, p_y);
    let mut p4 = CurvePoint::point_from_xy(&BLS12_381_G2_CURVE, p_x, p_y);
    */


    let gt_one = Fp12::one(&BLS12_381_EXTENSION_12_FIELD);



	// convert input to point objects
	//let mut p = slice_of_input();
	//let mut q = slice_of_input():

    p.normalize();
    //q.normalize();

    //let res = BLS12_381_PAIRING_ENGINE.pair(&[p.clone(), p2.clone()], &[q.clone(), q2.clone()]).unwrap();
    //assert!(res == gt_one);
}
