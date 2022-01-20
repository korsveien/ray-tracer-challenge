use crate::equal;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn is_black(&self) -> bool {
        self.r == 0.0 && self.g == 0.0 && self.b == 0.0
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        equal(self.r, other.r) && equal(self.g, other.g) && equal(self.b, other.b)
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

#[test]
fn should_add_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);

    let expected = Color::new(1.6, 0.7, 1.0);
    let actual = c1 + c2;

    assert_eq!(expected, actual);
}

#[test]
fn should_subtract_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);

    let expected = Color::new(0.2, 0.5, 0.5);
    let actual = c1 - c2;

    assert_eq!(expected, actual);
}

#[test]
fn should_multiply_color_with_scalar() {
    let c = Color::new(0.2, 0.3, 0.4);

    let expected = Color::new(0.4, 0.6, 0.8);
    let actual = c * 2.0;

    assert_eq!(expected, actual);
}

#[test]
fn should_multiply_two_colors() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);

    let expected = Color::new(0.9, 0.2, 0.04);
    let actual = c1 * c2;

    assert_eq!(expected, actual);
}
