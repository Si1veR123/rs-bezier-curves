
pub mod bezier;
pub mod vector;
pub mod maths;
pub mod control_point;

use vector::Vector;
use bezier::{BezierCurve, GenericBezierCurve};
use control_point::ControlPoint;

pub trait Spline<const DIM: usize> {
    fn get_point_along_spline(&self, t: f64) -> Option<Vector<DIM>>;
}


pub struct GenericSpline<const DIM: usize> {
    pub curves: Vec<Box<dyn BezierCurve<DIM>>>
}

#[allow(dead_code)]
impl<const DIM: usize> GenericSpline<DIM> {
    pub fn from_curves(curves: Vec<Box<dyn BezierCurve<DIM>>>) -> Self {
        Self {curves}
    }

    fn cubic_bezier_from_points(points: [Vector<DIM>; 4]) -> Box<dyn BezierCurve<DIM>> {
        Box::new(GenericBezierCurve::new(points.to_vec())) as Box<dyn BezierCurve<DIM>>
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
