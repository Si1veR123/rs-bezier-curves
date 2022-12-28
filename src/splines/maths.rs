use super::vector::Vector;
use std::ops::Mul;

pub fn factorial(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        7 => 5040,
        8 => 40320,
        9 => 362880,
        10 => 3628800,
        11.. => (1..num+1).product(),
    }
}

pub fn differentiate_coefficients<const DIM: usize>(coefficients: &Vec<Vector<DIM>>) -> Vec<Vector<DIM>> {
    let mut new_coeffs: Vec<Vector<DIM>> = vec![];
    for (i, old_coeff) in coefficients[1..].iter().enumerate() {
        let new_coeff = old_coeff.clone() * (i+1) as f64;
        new_coeffs.push(new_coeff)
    }
    new_coeffs
}

pub fn evaluate_polynomial<const DIM: usize>(coefficients: &Vec<Vector<DIM>>, t: f64) -> Vector<DIM> {
    let mut new_vec = Vector {data: [0f64; DIM]};

    for i in 0..coefficients.len() {
        let term = coefficients.get(i).unwrap().clone().mul(t.powi(i as i32));
        new_vec = new_vec.add_vector(&term);
    }
    new_vec
}
