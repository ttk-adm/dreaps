#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }
}

impl<T> Matrix<T> {
    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(rows * cols, data.len(), "Data does not match dimensions");
        Self { rows, cols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = value;
        } else {
            panic!("Index out of bounds");
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl<T: Clone> Matrix<T> {
    pub fn get_row(&self, row: usize) -> Option<&[T]> {
        if row < self.rows {
            let start = row * self.cols;
            let end = start + self.cols;
            Some(&self.data[start..end])
        } else {
            None
        }
    }

    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row < self.rows {
            let start = row * self.cols;
            let end = start + self.cols;
            Some(&mut self.data[start..end])
        } else {
            None
        }
    }

    pub fn set_row(&mut self, row: usize, values: &[T]) {
        assert!(row < self.rows, "Row out of bounds");
        assert_eq!(values.len(), self.cols, "Row length mismatch");

        let start = row * self.cols;
        let end = start + self.cols;
        self.data[start..end].clone_from_slice(values);
    }

    pub fn get_column(&self, col: usize) -> Option<Vec<T>> {
        if col >= self.cols {
            return None;
        }

        let mut column = Vec::with_capacity(self.rows);
        for row in 0..self.rows {
            column.push(self.data[row * self.cols + col].clone());
        }
        Some(column)
    }

    pub fn set_column(&mut self, col: usize, values: &[T]) {
        assert!(col < self.cols, "Column out of bounds");
        assert_eq!(values.len(), self.rows, "Column length mismatch");

        for row in 0..self.rows {
            self.data[row * self.cols + col] = values[row].clone();
        }
    }
}

impl Matrix<f64> {
    pub fn dot(&self, other: &Matrix<f64>) -> Matrix<f64> {
        assert_eq!(self.cols, other.rows, "Incompatible matrix dimensions");

        let mut result = Matrix::new(self.rows, other.cols);

        for i in 0..self.rows {
            let row_offset = i * self.cols;

            for j in 0..other.cols {
                let mut sum = 0.0;

                for k in 0..self.cols {
                    sum += self.data[row_offset + k] * other.data[k * other.cols + j];
                }

                result.data[i * result.cols + j] = sum;
            }
        }

        result
    }

    pub fn determinant(&self) -> f64 {
        assert_eq!(self.rows, self.cols, "Determinant requires a square matrix");

        let n = self.rows;
        let mut mat = self.data.clone();
        let mut det = 1.0;

        for i in 0..n {
            // Find pivot
            let mut pivot = i;
            for j in (i + 1)..n {
                if mat[j * n + i].abs() > mat[pivot * n + i].abs() {
                    pivot = j;
                }
            }

            // If pivot is zero → determinant is zero
            if mat[pivot * n + i].abs() < 1e-12 {
                return 0.0;
            }

            // Swap rows if needed
            if i != pivot {
                for k in 0..n {
                    mat.swap(i * n + k, pivot * n + k);
                }
                det = -det; // row swap flips sign
            }

            let pivot_val = mat[i * n + i];
            det *= pivot_val;

            // Eliminate below
            for j in (i + 1)..n {
                let factor = mat[j * n + i] / pivot_val;
                for k in (i + 1)..n {
                    mat[j * n + k] -= factor * mat[i * n + k];
                }
            }
        }

        det
    }
}

pub fn dot(matrix_a: &[Vec<f64>], matrix_b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    assert_eq!(matrix_b.len(), matrix_a[0].len());
    let rows: usize = matrix_a.len();
    let cols: usize = matrix_b[0].len();

    (0..rows)
        .map(|row: usize| {
            (0..cols)
                .map(|col: usize| {
                    matrix_a[row]
                        .iter()
                        .zip(matrix_b.iter())
                        .map(|(a_nm, b_m)| a_nm * b_m[col])
                        .sum()
                })
                .collect()
        })
        .collect()
}
