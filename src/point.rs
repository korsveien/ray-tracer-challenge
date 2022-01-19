use crate::vector::Vector;
use crate::{equal, Tuple};
use std::ops;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        0.0
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x) && equal(self.y, other.y) && equal(self.z, other.z)
    }
}

impl ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl ops::Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self {
        Self {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[test]
fn should_subtract_two_points() {
    let p1 = Point::new(3.0, 2.0, 1.0);
    let p2 = Point::new(5.0, 6.0, 7.0);

    let expected = Vector::new(-2.0, -4.0, -6.0);
    let actual = p1 - p2;

    assert_eq!(expected, actual);
}

#[test]
fn should_create_tuple_that_is_a_point() {
    assert_eq!(
        Point::new(1.0, 1.0, 1.0),
        Point {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    )
}

#[test]
fn should_subtract_vector_from_point() {
    let p1 = Point::new(3.0, 2.0, 1.0);
    let p2 = Vector::new(5.0, 6.0, 7.0);

    let expected = Point::new(-2.0, -4.0, -6.0);
    let actual = p1 - p2;

    assert_eq!(expected, actual);
}

#[test]
fn should_multiply_a_point_by_a_scalar() {
    let a = Point::new(1.0, -2.0, 3.0);

    let expected = Point::new(3.5, -7.0, 10.5);
    let actual = a * 3.5;

    assert_eq!(expected, actual);
}

#[test]
fn should_negate_a_point() {
    let a = Point::new(1.0, -2.0, 3.0);

    let expected = Point::new(-1.0, 2.0, -3.0);
    let actual = -a;

    assert_eq!(expected, actual);
}

#[test]
fn should_divide_a_point_by_a_scalar() {
    let a = Point::new(1.0, -2.0, 3.0);

    let expected = Point::new(0.5, -1.0, 1.5);
    let actual = a / 2.0;

    assert_eq!(expected, actual);
}
