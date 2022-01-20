use crate::{equal, Tuple};
use std::ops;

#[derive(Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn magnitude(&self) -> f64 {
        let sum_of_squares = self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0);
        sum_of_squares.sqrt()
    }

    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }
}

impl Tuple for Vector {
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

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x) && equal(self.y, other.y) && equal(self.z, other.z)
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Div<f64> for Vector {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[test]
fn should_create_tuple_that_is_a_vector() {
    assert_eq!(
        Vector::new(1.0, 1.0, 1.0),
        Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    )
}

#[test]
fn should_subtract_two_vectors() {
    let v1 = Vector::new(3.0, 2.0, 1.0);
    let v2 = Vector::new(5.0, 6.0, 7.0);

    let expected = Vector::new(-2.0, -4.0, -6.0);
    let actual = v1 - v2;

    assert_eq!(expected, actual);
}

#[test]
fn should_negate_a_vector() {
    let a = Vector::new(1.0, -2.0, 3.0);

    let expected = Vector::new(-1.0, 2.0, -3.0);
    let actual = -a;

    assert_eq!(expected, actual);
}

#[test]
fn should_multiply_a_vector_by_a_scalar() {
    let a = Vector::new(1.0, -2.0, 3.0);

    let expected = Vector::new(3.5, -7.0, 10.5);
    let actual = a * 3.5;

    assert_eq!(expected, actual);
}

#[test]
fn should_divide_a_vector_by_a_scalar() {
    let a = Vector::new(1.0, -2.0, 3.0);

    let expected = Vector::new(0.5, -1.0, 1.5);
    let actual = a / 2.0;

    assert_eq!(expected, actual);
}

#[test]
fn should_compute_the_magnitude_of_a_vector() {
    let v = Vector::new(1.0, 0.0, 0.0);

    let expected = 1.0;
    let actual = v.magnitude();

    assert_eq!(expected, actual);

    let v = Vector::new(0.0, 1.0, 0.0);

    let expected = 1.0;
    let actual = v.magnitude();

    assert_eq!(expected, actual);

    let v = Vector::new(0.0, 0.0, 1.0);

    let expected = 1.0;
    let actual = v.magnitude();

    assert_eq!(expected, actual);

    let v = Vector::new(1.0, 2.0, 3.0);

    let expected = 14.0_f64.sqrt();
    let actual = v.magnitude();

    assert_eq!(expected, actual);

    let v = Vector::new(1.0, 2.0, 3.0);

    let expected = 14.0_f64.sqrt();
    let actual = v.magnitude();

    assert_eq!(expected, actual);

    let v = Vector::new(-1.0, -2.0, -3.0);

    let expected = 14.0_f64.sqrt();
    let actual = v.magnitude();

    assert_eq!(expected, actual);
}

#[test]
fn should_normalize_a_vector() {
    let v = Vector::new(4.0, 0.0, 0.0);

    let expected = Vector::new(1.0, 0.0, 0.0);
    let actual = v.normalize();

    assert_eq!(expected, actual);

    let v = Vector::new(1.0, 2.0, 3.0);

    let expected = Vector::new(0.26726, 0.53452, 0.80178);
    let actual = v.normalize();

    assert_eq!(expected, actual);
}
