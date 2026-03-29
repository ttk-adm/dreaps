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
