use rayon::prelude::*;

pub fn dot_thr(matrix_a: &[Vec<f64>], matrix_b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    assert_eq!(matrix_b.len(), matrix_a[0].len());
    let cols = matrix_b[0].len();

    matrix_a
        .par_iter()
        .map(|row| {
            (0..cols)
                .map(|col| {
                    row.iter()
                        .zip(matrix_b.iter())
                        .map(|(a_ik, b_k)| a_ik * b_k[col])
                        .sum()
                })
                .collect()
        })
        .collect()
}
