use crate::equal;

#[derive(Debug, Clone)]
struct Matrix<'a> {
    entries: &'a Vec<Vec<f64>>,
}

impl<'a> Matrix<'a> {
    pub fn with_entries(entries: &Vec<Vec<f64>>) -> Matrix {
        assert!(!entries.is_empty());

        let len = entries.get(0).unwrap().len();
        let is_quadratical = entries.iter().all(|row| row.len() == len);
        assert!(is_quadratical);

        Matrix { entries }
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&f64> {
        self.entries.get(row).and_then(|row| row.get(column))
    }
}

impl<'a> PartialEq for Matrix<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.entries.len() != other.entries.len() {
            return false;
        }

        self.entries
            .iter()
            .zip(other.entries.iter())
            .all(|(row, other_row)| row.iter().zip(other_row.iter()).all(|(a, b)| equal(*a, *b)))
    }
}

#[test]
#[should_panic]
fn should_panic_if_not_quadratical() {
    let entries = vec![vec![0.0], vec![0.0, 0.0]];
    Matrix::with_entries(&entries);
}

#[test]
fn should_get_values_out_of_matrix() {
    let entries = vec![vec![-3.0, -5.0], vec![1.0, -2.0]];
    let m = Matrix::with_entries(&entries);

    assert_eq!(*m.get(0, 0).unwrap(), -3.0);
    assert_eq!(*m.get(0, 1).unwrap(), -5.0);
    assert_eq!(*m.get(1, 0).unwrap(), 1.0);
    assert_eq!(*m.get(1, 1).unwrap(), -2.0);
}

#[test]
fn should_compare_equality() {
    let entries = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![9.0, 8.0, 7.0, 6.0],
        vec![5.0, 4.0, 3.0, 2.0],
    ];

    let m1 = Matrix::with_entries(&entries);
    let m2 = Matrix::with_entries(&entries);

    assert_eq!(m1, m2);
}
