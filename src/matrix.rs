use std::ops;

type Matrix4 = [[f64; 4]; 4];
type Matrix3 = [[f64; 3]; 3];
type Matrix2 = [[f64; 2]; 2];

#[derive(Debug, Copy, Clone, PartialEq)]
struct Matrix {
    entries: Matrix4,
}

impl Matrix {
    fn with_entries(entries: Matrix4) -> Matrix {
        Matrix { entries }
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, other: Matrix) -> Self {
        let mut entries: Matrix4 = [[0.0; 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                entries[row][col] = self.entries[row][0] * other.entries[0][col]
                    + self.entries[row][1] * other.entries[1][col]
                    + self.entries[row][2] * other.entries[2][col]
                    + self.entries[row][3] * other.entries[3][col];
            }
        }

        Matrix { entries }
    }
}

#[test]
fn should_construct_and_inspect_a_4_by_4_matrix() {
    let m: Matrix4 = [
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
    ];

    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][3], 4.0);
    assert_eq!(m[1][0], 5.5);
    assert_eq!(m[1][2], 7.5);
    assert_eq!(m[2][2], 11.0);
    assert_eq!(m[3][0], 13.5);
    assert_eq!(m[3][2], 15.5);
}

#[test]
fn should_construct_and_inspect_a_2_by_2_matrix() {
    let m: Matrix2 = [[-3.0, 5.0], [1.0, -2.0]];

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[0][1], 5.0);
    assert_eq!(m[1][0], 1.0);
    assert_eq!(m[1][1], -2.0);
}

#[test]
fn should_construct_and_inspect_a_3_by_3_matrix() {
    let m: Matrix3 = [[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]];

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[1][1], -2.0);
    assert_eq!(m[2][2], 1.0);
}

#[test]
fn should_compare_identical_matrices() {
    let a: Matrix4 = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ];

    let b: Matrix4 = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ];

    assert_eq!(a, b);
}

#[test]
fn should_compare_different_matrices() {
    let a: Matrix4 = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ];

    let b: Matrix4 = [
        [2.0, 3.0, 4.0, 5.0],
        [6.0, 7.0, 8.0, 9.0],
        [8.0, 7.0, 6.0, 5.0],
        [4.0, 3.0, 2.0, 1.0],
    ];

    assert_ne!(a, b);
}

#[test]
fn should_multiply_two_matrices() {
    let a = Matrix::with_entries([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    let b = Matrix::with_entries([
        [-2.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
    ]);

    let expected = Matrix::with_entries([
        [20.0, 22.0, 50.0, 48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0, 46.0, 42.0],
    ]);

    let actual = a * b;

    assert_eq!(expected, actual);
}
