
pub mod bezier;
pub mod vector;
pub mod maths;

use vector::Vector;
use bezier::{BezierCurve, GenericBezierCurve};

pub trait Spline<const DIM: usize> {
    fn get_point_along_spline(&self, t: f64) -> Option<Vector<DIM>>;
}


pub struct GenericSpline<const DIM: usize> {
    pub curves: Vec<Box<dyn BezierCurve<DIM>>>
}

impl<const DIM: usize> GenericSpline<DIM> {
    pub fn from_curves(curves: Vec<Box<dyn BezierCurve<DIM>>>) -> Self {
        Self {curves}
    }

    pub fn cubic_from_points(points: &Vec<Vector<DIM>>) -> Option<Self> {
        let mut curves = vec![];
        for curve_index in 0..points.len()-3 {
            let point1 = points.get(curve_index)?.clone();
            let point2 = points.get(curve_index+1)?.clone();
            let point3 = points.get(curve_index+2)?.clone();
            let point4 = points.get(curve_index+3)?.clone();
            
            let curve = Box::new(GenericBezierCurve::new(vec![point1, point2, point3, point4])) as Box<dyn BezierCurve<DIM>>;
            curves.push(curve);
        }
        Some(Self {curves})
    }
}

impl<const DIM: usize> Spline<DIM> for GenericSpline<DIM> {
    fn get_point_along_spline(&self, t: f64) -> Option<Vector<DIM>> {
        if t == 1.0 {
            return Some(self.curves.last()?.get_point_along_curve(1.0))
        }

        let t_unnormalised = t * self.curves.len() as f64;
        let curve_index = t_unnormalised.trunc() as usize;
        let curve_relative_t = t_unnormalised.fract();
        let curve = self.curves.get(curve_index)?;
        Some(curve.get_point_along_curve(curve_relative_t))
    }
}
