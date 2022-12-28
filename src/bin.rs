mod splines;
use splines::{GenericSpline, Spline};
use splines::vector::Vector;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let points : Vec<Vector<2, f64>>= vec![
        Vector {data: [0.0, 0.0]},
        Vector {data: [2.0, 2.0]},
        Vector {data: [4.0, 2.0]},
        Vector {data: [6.0, 0.0]},
        Vector {data: [8.0, -2.0]},
        Vector {data: [8.0, -4.0]},
        Vector {data: [6.0, -6.0]},
    ];
    let spline = GenericSpline::cubic_from_points(&points).unwrap();
    println!("{}", spline.curves.len());
    println!("{:?}", spline.get_point_along_spline(0.25).unwrap());
    println!("{:?}", Instant::now() - start);
}
