pub fn compare_len(array_a: &[f64], array_b: &[f64]) -> Result<(), String> {
    if array_a.len() != array_b.len() {
        Err("arrays must have equal lengths".into())
    } else {
        Ok(())
    }
}
