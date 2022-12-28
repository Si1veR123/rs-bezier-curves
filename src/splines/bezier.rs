use std::ops::Mul;

use super::vector::Vector;
use super::maths::{factorial, differentiate_coefficients, evaluate_polynomial};
use super::control_point::ControlPoint;

// ####################################
// ########## CURVES SECTION ##########
// ####################################

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
    fn get_point_along_curve(&self, t: f64) -> Option<Vector<DIM>>;
    fn get_tangent_along_curve(&self, t: f64) -> Option<Vector<DIM>>;
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
    fn get_point_along_curve(&self, t: f64) -> Option<Vector<DIM>> {
        Some(evaluate_polynomial(&self.coefficients, t))
    }

    fn get_tangent_along_curve(&self, t: f64) -> Option<Vector<DIM>> {
        let differentiated_polynomial = differentiate_coefficients(&self.coefficients);
        Some(evaluate_polynomial(&differentiated_polynomial, t))
    }
}


// ####################################
// ########## SPLINE SECTION ##########
// ####################################
// A spline is multiple curves joined together, usually cubic bezier curves

pub struct GenericBezierSpline<const DIM: usize> {
    pub curves: Vec<Box<dyn BezierCurve<DIM>>>
}

#[allow(dead_code)]
impl<const DIM: usize> GenericBezierSpline<DIM> {
    pub fn from_curves(curves: Vec<Box<dyn BezierCurve<DIM>>>) -> Self {
        Self {curves}
    }

    fn cubic_bezier_from_points(points: [Vector<DIM>; 4]) -> Box<dyn BezierCurve<DIM>> {
        Box::new(GenericBezierCurve::new(points.to_vec())) as Box<dyn BezierCurve<DIM>>
    }

    fn get_curve_relative_t(&self, t: f64) -> (usize, f64) {
        if t == 1.0 {
            return (self.curves.len()-1, 1.0)
        }

        let t_unnormalised = t * self.curves.len() as f64;
        let curve_index = t_unnormalised.trunc() as usize;
        let curve_relative_t = t_unnormalised.fract();

        (curve_index, curve_relative_t)
    }

    pub fn cubic_from_points(points: &Vec<Vector<DIM>>) -> Option<Self> {
        // pattern of points is [control point1, handle, handle, control point2, handle, handle, control point3 ...]
        if (points.len()-1)%3 != 0 {
            return None
        }

        let mut curves = vec![];
        for curve_index in 0..(points.len()-1)/3 {
            let start_index = curve_index*3;
            let point1 = points.get(start_index)?.clone();
            let point2 = points.get(start_index+1)?.clone();
            let point3 = points.get(start_index+2)?.clone();
            let point4 = points.get(start_index+3)?.clone();
            
            let curve = Self::cubic_bezier_from_points([point1, point2, point3, point4]);
            curves.push(curve);
        }
        Some(Self {curves})
    }

    pub fn cubic_from_control_points(points: &Vec<Box<dyn ControlPoint<DIM>>>) -> Self {
        // first handle of first point and last handle of last point arent used
        let mut curves = vec![];
        
        for curve_index in 0..points.len()-1 {
            let point1 = points.get(curve_index).unwrap().get_control_point_position();
            let point2 = points.get(curve_index).unwrap().get_second_handle();
            let point3 = points.get(curve_index+1).unwrap().get_first_handle();
            let point4 = points.get(curve_index+1).unwrap().get_control_point_position();

            let curve = Self::cubic_bezier_from_points([point1, point2, point3, point4]);
            curves.push(curve);
        }

        Self {curves}
    }
}

impl<const DIM: usize> BezierCurve<DIM> for GenericBezierSpline<DIM> {
    fn get_point_along_curve(&self, t: f64) -> Option<Vector<DIM>> {
        let (curve_index, curve_relative_t) = self.get_curve_relative_t(t);
        let curve = self.curves.get(curve_index)?;
        Some(curve.get_point_along_curve(curve_relative_t)?)
    }
    fn get_tangent_along_curve(&self, t: f64) -> Option<Vector<DIM>> {
        let (curve_index, curve_relative_t) = self.get_curve_relative_t(t);
        let curve = self.curves.get(curve_index)?;
        Some(curve.get_tangent_along_curve(curve_relative_t)?)
    }
}

