pub mod app;
pub mod date;

pub fn min(series: &[f64]) -> Option<f64> {
    filter_nan(series)
        .min_by(cmp_f64())
        .cloned()
}

pub fn max(series: &[f64]) -> Option<f64> {
    filter_nan(series)
        .max_by(cmp_f64())
        .cloned()
}

fn filter_nan(series: &[f64]) -> impl Iterator<Item = &f64> {
    series.iter().filter(|v| !v.is_nan())
}

fn cmp_f64() -> Box<dyn FnMut(&&f64, &&f64) -> std::cmp::Ordering> {
    Box::new(|x: &&f64, y: &&f64| x.partial_cmp(y).unwrap())
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
        fn ignore_nan() {
            let series: Vec<f64> = vec![2.1, std::f64::NAN, 3.6];
            assert_eq!(Some(2.1), min(&series[..]));
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
        fn ignore_nan() {
            let series: Vec<f64> = vec![2.1, std::f64::NAN, 3.6];
            assert_eq!(Some(3.6), max(&series[..]));
        }
    }
}
