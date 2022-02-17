use crate::equal;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        let sum_of_squares = self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0);
        sum_of_squares.sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x) && equal(self.y, other.y) && equal(self.z, other.z)
    }
}

impl From<[f64; 3]> for Vector {
    fn from(array: [f64; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
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

    let v = Vector::new(1.0, 2.0, 3.0);

    let expected = 1.0;
    let actual = v.normalize().magnitude();

    assert_eq!(expected, actual);
}

#[test]
fn should_calculate_dot_product_of_two_vectors() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(2.0, 3.0, 4.0);

    let expected = 20.0;
    let actual = a.dot(&b);

    assert_eq!(expected, actual);
}

#[test]
fn should_calculate_cross_product_of_two_vectors() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(2.0, 3.0, 4.0);

    let expected = Vector::new(-1.0, 2.0, -1.0);
    let actual = a.cross(&b);

    assert_eq!(expected, actual);

    let expected = Vector::new(1.0, -2.0, 1.0);
    let actual = b.cross(&a);

    assert_eq!(expected, actual);
}
