use std::ops::Mul;

use super::vector::Vector;
use super::maths::factorial;

fn generate_coefficients<const DIM: usize>(points: &Vec<Vector<DIM>>) -> Vec<Vector<DIM>> {
    // https://en.wikipedia.org/wiki/B%C3%A9zier_curve
    // polynomial definition
    if points.len() == 0 {
        return vec![]
    }

    let mut coeffs = vec![points.get(0).unwrap().clone()];
    let n = points.len()-1;

    for j in 1..=n {
        let first_multiplier: f64 = factorial(n as u128) as f64 / factorial((n - j) as u128) as f64;
        let mut summation = Vector {data: [0f64; DIM]};
        for i in 0..=j {
            let numerator_multiplier = (-1i8).pow((i+j) as u32);
            let numerator = points[i].clone().mul(numerator_multiplier as f64);
            let denominator = factorial(i as u128) * factorial((j - i) as u128);
            let value = numerator * (1f64/(denominator as f64));
            summation = summation.add_vector(&value);
        }
        coeffs.push(summation * first_multiplier);
    }

    coeffs
}

pub trait BezierCurve<const DIM: usize> {
    fn get_point_along_curve(&self, t: f64) -> Vector<DIM>;
}

#[derive(Debug)]
pub struct GenericBezierCurve<const DIM: usize> {
    // will overflow at around 34 points, not recommended for high polynomials due to numerical instability in polynomial method
    pub points: Vec<Vector<DIM>>,
    pub coefficients: Vec<Vector<DIM>>
}

impl<const DIM: usize> GenericBezierCurve<DIM> {
    pub fn new(points: Vec<Vector<DIM>>) -> Self {
        let coefficients = generate_coefficients(&points);
        Self { points, coefficients }
    }
}

impl<const DIM: usize> BezierCurve<DIM> for GenericBezierCurve<DIM> {
    fn get_point_along_curve(&self, t: f64) -> Vector<DIM> {
        let mut new_vec = Vector {data: [0f64; DIM]};
        // evaluate polynomial
        for i in 0..self.points.len() {
            let term = self.coefficients.get(i).unwrap().clone().mul(t.powi(i as i32));
            new_vec = new_vec.add_vector(&term);
        }
        new_vec
    }
}
