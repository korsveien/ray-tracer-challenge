use std::ops;

type Matrix4 = [[f64; 4]; 4];
type Matrix3 = [[f64; 3]; 3];
type Matrix2 = [[f64; 2]; 2];
type Tuple = [f64; 4];

#[derive(Debug, Copy, Clone, PartialEq)]
struct Matrix {
    entries: Matrix4,
}

impl Matrix {
    fn with_entries(entries: Matrix4) -> Self {
        Self { entries }
    }

    fn transpose(&self) -> Self {
        let mut entries: Matrix4 = [[0.0; 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                entries[col][row] = self.entries[row][col]
            }
        }
        Self { entries }
    }

    fn determinant(m: Matrix2) -> f64 {
        m[0][0] * m[1][1] - m[1][0] * m[0][1]
    }

    fn submatrix_3_to_2(m: Matrix3, row: usize, col: usize) -> Matrix2 {
        let mut submatrix = [[0.0; 2]; 2];
        let mut i = 0;
        let mut j = 0;

        for (index, element) in m.iter().enumerate() {
            if index == row {
                continue;
            }
            for (index, entry) in element.iter().enumerate() {
                if index == col {
                    continue;
                }

                submatrix[i][j] = *entry;

                j = match j {
                    1 => 0,
                    _ => 1,
                };
            }

            i = match i {
                1 => 0,
                _ => 1,
            };
        }
        submatrix
    }

    fn submatrix_4_to_3(m: Matrix4, row: usize, col: usize) -> Matrix3 {
        let mut submatrix = [[0.0; 3]; 3];
        let mut i = 0;
        let mut j = 0;

        for (index, element) in m.iter().enumerate() {
            if index == row {
                continue;
            }
            for (index, entry) in element.iter().enumerate() {
                if index == col {
                    continue;
                }

                submatrix[i][j] = *entry;

                j = match j {
                    2 => 0,
                    _ => j + 1,
                };
            }

            i = match i {
                2 => 0,
                _ => i + 1,
            };
        }
        submatrix
    }

    fn minor(m: Matrix3, row: usize, col: usize) -> f64 {
        let submatrix = Matrix::submatrix_3_to_2(m, row, col);
        Matrix::determinant(submatrix)
    }

    fn cofactor(m: Matrix3, row: usize, col: usize) -> f64 {
        let minor = Matrix::minor(m, row, col);
        if row + col % 2 == 0 {
            minor
        } else {
            -minor
        }
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

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let mut result: Tuple = [0.0; 4];
        for row in 0..4 {
            result[row] = self.entries[row][0] * other[0]
                + self.entries[row][1] * other[1]
                + self.entries[row][2] * other[2]
                + self.entries[row][3] * other[3];
        }

        result
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

#[test]
fn should_multiply_a_matrix_with_a_tuple() {
    let a = Matrix::with_entries([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let b: Tuple = [1.0, 2.0, 3.0, 1.0];

    let expected: Tuple = [18.0, 24.0, 33.0, 1.0];
    let actual: Tuple = a * b;
    assert_eq!(expected, actual);
}

#[test]
fn should_multiply_matrix_by_the_identity_matrix() {
    let a = Matrix::with_entries([
        [0.0, 1.0, 2.0, 4.0],
        [1.0, 2.0, 4.0, 8.0],
        [2.0, 4.0, 8.0, 16.0],
        [4.0, 8.0, 16.0, 32.0],
    ]);

    let identity_matrix = Matrix::with_entries([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let actual = a * identity_matrix;
    assert_eq!(a, actual);
}

#[test]
fn should_multiply_matrix_by_a_tuple() {
    let a: Tuple = [1.0, 2.0, 3.0, 4.0];

    let identity_matrix = Matrix::with_entries([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let actual = identity_matrix * a;
    assert_eq!(a, actual);
}

#[test]
fn should_transpose_a_matrix() {
    let a = Matrix::with_entries([
        [0.0, 9.0, 3.0, 0.0],
        [9.0, 8.0, 0.0, 8.0],
        [1.0, 8.0, 5.0, 3.0],
        [0.0, 0.0, 5.0, 8.0],
    ]);

    let expected = Matrix::with_entries([
        [0.0, 9.0, 1.0, 0.0],
        [9.0, 8.0, 8.0, 0.0],
        [3.0, 0.0, 5.0, 5.0],
        [0.0, 8.0, 3.0, 8.0],
    ]);

    assert_eq!(expected, a.transpose());
}

#[test]
fn should_transpose_identity_matrix() {
    let identity_matrix = Matrix::with_entries([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    assert_eq!(identity_matrix, identity_matrix.transpose());
}

#[test]
fn should_calculate_the_determinant() {
    let a: Matrix2 = [[1.0, 5.0], [-3.0, 2.0]];
    let expected = 17.0;

    let actual = Matrix::determinant(a);

    assert_eq!(expected, actual);
}

#[test]
fn should_return_a_2_x_2_submatrix() {
    let a: Matrix3 = [[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]];
    let expected = [[-3.0, 2.0], [0.0, 6.0]];
    let actual = Matrix::submatrix_3_to_2(a, 0, 2);
    assert_eq!(expected, actual);
}

#[test]
fn should_return_a_3_x_3_submatrix() {
    let a: Matrix4 = [
        [-6.0, 1.0, 1.0, 6.0],
        [-8.0, 5.0, 8.0, 6.0],
        [-1.0, 0.0, 8.0, 2.0],
        [-7.0, 1.0, -1.0, 1.0],
    ];
    let expected = [[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]];
    let actual = Matrix::submatrix_4_to_3(a, 2, 1);
    assert_eq!(expected, actual);
}

#[test]
fn should_return_minor_of_3_by_3_matrix() {
    let a: Matrix3 = [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]];
    assert_eq!(Matrix::minor(a, 1, 0), 25.0);
}

#[test]
fn should_return_cofactor_of_3_by_3_matrix() {
    let a: Matrix3 = [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]];
    assert_eq!(Matrix::cofactor(a, 0, 0), -12.0);
    assert_eq!(Matrix::cofactor(a, 1, 0), -25.0);
}
