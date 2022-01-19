mod point;
mod vector;

fn equal(f1: f64, f2: f64) -> bool {
    const EPSILON: f64 = 0.00001;
    if (f1 - f2).abs() < EPSILON {
        return true;
    }
    false
}

trait Tuple {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}
