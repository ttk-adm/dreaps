#[cfg(test)]
mod tests {
    use crate::math::matrix::dot;
    use crate::math::matrix_thr::dot_thr;
    use std::time::Instant;

    #[test]
    fn test_dot() {
        let matrix_a: Vec<Vec<f64>> = vec![
            vec![1., 0., 1.],
            vec![2., 1., 1.],
            vec![0., 1., 1.],
            vec![1., 1., 2.],
        ];
        let matrix_b: Vec<Vec<f64>> = vec![vec![1., 2., 1.], vec![2., 3., 1.], vec![4., 2., 2.]];
        let dot_product: Vec<Vec<f64>> = dot(&matrix_a, &matrix_b);
        let exp_product: Vec<Vec<f64>> = vec![
            vec![5., 4., 3.],
            vec![8., 9., 5.],
            vec![6., 5., 3.],
            vec![11., 9., 6.],
        ];
        assert_eq!(dot_product, exp_product);
    }

    #[test]
    #[should_panic]
    fn test_dot_shape_mismatch() {
        let matrix_a: Vec<Vec<f64>> = vec![vec![1.; 4]; 4];
        let matrix_b: Vec<Vec<f64>> = vec![vec![1.; 2]; 2];
        dot(&matrix_a, &matrix_b);
    }

    #[test]
    fn test_dot_thr() {
        let matrix_a: Vec<Vec<f64>> = vec![
            vec![1., 0., 1.],
            vec![2., 1., 1.],
            vec![0., 1., 1.],
            vec![1., 1., 2.],
        ];
        let matrix_b: Vec<Vec<f64>> = vec![vec![1., 2., 1.], vec![2., 3., 1.], vec![4., 2., 2.]];
        let dot_product: Vec<Vec<f64>> = dot_thr(&matrix_a, &matrix_b);
        let exp_product: Vec<Vec<f64>> = vec![
            vec![5., 4., 3.],
            vec![8., 9., 5.],
            vec![6., 5., 3.],
            vec![11., 9., 6.],
        ];
        assert_eq!(dot_product, exp_product);
    }

    #[test]
    #[should_panic]
    fn test_dot_thr_shape_mismatch() {
        let matrix_a: Vec<Vec<f64>> = vec![vec![1.; 4]; 4];
        let matrix_b: Vec<Vec<f64>> = vec![vec![1.; 2]; 2];
        dot_thr(&matrix_a, &matrix_b);
    }

    #[test]
    #[ignore]
    fn test_times() {
        let matrix_a: Vec<Vec<f64>> = vec![
            vec![1., 0., 1.],
            vec![2., 1., 1.],
            vec![0., 1., 1.],
            vec![1., 1., 2.],
        ];
        let matrix_b: Vec<Vec<f64>> = vec![vec![1., 2., 1.], vec![2., 3., 1.], vec![4., 2., 2.]];
        let fnls = [dot, dot_thr];
        let times: Vec<f64> = fnls
            .iter()
            .map(|_fn| {
                let t0: Instant = Instant::now();
                for _ in 0..10_000 {
                    _fn(&matrix_a, &matrix_b);
                }
                t0.elapsed().as_micros() as f64
            })
            .collect();
        let diffper: f64 = (times[1] / times[0]) * 100.;
        println!(
            "Multi-Threading time is {:.2} % of the single-threaded method",
            diffper
        );
        println!(
            "Multi-Threaded: {} us\nSingle-Threaded: {} us",
            times[1], times[0]
        );

        let matrix_a: Vec<Vec<f64>> = (0..12)
            .map(|n| (0..12).map(|m| (n + m) as f64).collect())
            .collect();
        let matrix_b: Vec<Vec<f64>> = (0..12)
            .map(|n| (0..12).map(|m| (n + m) as f64).collect())
            .collect();
        let times: Vec<f64> = fnls
            .iter()
            .map(|_fn| {
                let t0: Instant = Instant::now();
                for _ in 0..10_000 {
                    _fn(&matrix_a, &matrix_b);
                }
                t0.elapsed().as_micros() as f64
            })
            .collect();
        let diffper: f64 = (times[1] / times[0]) * 100.;
        println!(
            "Multi-Threading time is {:.2} % of the single-threaded method",
            diffper
        );
        println!(
            "Multi-Threaded: {} us\nSingle-Threaded: {} us",
            times[1], times[0]
        );
    }
}
