pub mod app;
pub mod date;

pub fn min(series: &[f64]) -> Option<f64> {
    if has_nan(series) {
        None
    } else {
        series.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| *x)
    }
}

pub fn max(series: &[f64]) -> Option<f64> {
    if has_nan(series) {
        None
    } else {
        series.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| *x)
    }
}

fn has_nan(series: &[f64]) -> bool {
    series.iter().any(|v| v.is_nan())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fn_min {
        use super::*;

        #[test]
        fn return_minimum_if_not_empty() {
            let series: Vec<f64> = vec![2.1, 1.3, 3.6];
            assert_eq!(Some(1.3), min(&series[..]));
        }

        #[test]
        fn return_none_if_empty() {
            let series: Vec<f64> = vec![];
            assert_eq!(None, min(&series[..]));
        }

        #[test]
        fn return_none_if_contains_nan() {
            let series: Vec<f64> = vec![2.1, std::f64::NAN, 3.6];
            assert_eq!(None, min(&series[..]));
        }
    }

    mod fn_max {
        use super::*;

        #[test]
        fn return_maximum_if_not_empty() {
            let series: Vec<f64> = vec![2.1, 1.3, 3.6];
            assert_eq!(Some(3.6), max(&series[..]));
        }

        #[test]
        fn return_none_if_empty() {
            let series: Vec<f64> = vec![];
            assert_eq!(None, max(&series[..]));
        }

        #[test]
        fn return_none_if_contains_nan() {
            let series: Vec<f64> = vec![2.1, std::f64::NAN, 3.6];
            assert_eq!(None, max(&series[..]));
        }
    }

    mod fn_has_nan {
        use super::*;

        #[test]
        fn return_true_if_includes_nan() {
            let series: Vec<f64> = vec![2.1, std::f64::NAN, 3.6];
            assert!(has_nan(&series[..]))
        }

        #[test]
        fn return_false_if_not_include_nan() {
            let series: Vec<f64> = vec![2.1, 1.3, 3.6];
            assert!(!has_nan(&series[..]))
        }

        #[test]
        fn return_false_if_empty() {
            let series: Vec<f64> = vec![];
            assert!(!has_nan(&series[..]))
        }
    }
}
