mod splines;
use splines::vector::Vector;
use splines::bezier::{GenericBezierCurve, GenericBezierSpline, BezierCurve};
use splines::control_point::{AlignedControlPoint, MirroredControlPoint, CustomControlPoint, ControlPoint};

fn main() {
    // using raw points
    let points: Vec<Vector<2, f64>>= vec![
        Vector {data: [0.0, 0.0]},
        Vector {data: [2.0, 2.0]},
        Vector {data: [4.0, 2.0]},
        Vector {data: [6.0, 0.0]},
        Vector {data: [8.0, -2.0]},
        Vector {data: [8.0, -4.0]},
        Vector {data: [6.0, -6.0]},
    ];
    let spline = GenericBezierSpline::cubic_from_points(&points).unwrap();
    println!("{}", spline.curves.len());
    println!("{:?}", spline.get_point_along_curve(0.95).unwrap());
    
    // using all control points types (same spline)
    let control_points: Vec<Box<dyn ControlPoint<2>>> = vec![
        Box::new(CustomControlPoint::new(Vector {data: [0.0, 0.0]}, Vector {data: [0.0, 0.0]}, Vector { data: [2.0, 2.0] })),

        Box::new(MirroredControlPoint::new(Vector {data: [6.0, 0.0]}, Vector {data: [-2.0, 2.0]})),

        Box::new(AlignedControlPoint::new(Vector {data: [6.0, -6.0]}, Vector { data: [1.0, 1.0] }, 2.0, 0.0)),
    ];
    let control_point_spline = GenericBezierSpline::cubic_from_control_points(&control_points);
    println!("{:?}", control_point_spline.get_point_along_curve(0.95).unwrap());

    let points: Vec<Vector<2, f64>>= vec![
        Vector {data: [0.0, 0.0]},
        Vector {data: [2.0, 2.0]},
        Vector {data: [4.0, 2.0]},
        Vector {data: [6.0, 0.0]},
    ];
    let curve = GenericBezierCurve::new(points);
    println!("{:?}", curve);
    println!("{:?}", curve.get_tangent_along_curve(0.00).unwrap());
}
