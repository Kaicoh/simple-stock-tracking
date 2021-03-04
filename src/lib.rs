pub mod app;
pub mod date;

pub fn min(series: &[f64]) -> Option<f64> {
    filter_nan(series).min_by(cmp_f64())
}

pub fn max(series: &[f64]) -> Option<f64> {
    filter_nan(series).max_by(cmp_f64())
}

pub fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    let series: Vec<f64> = filter_nan(series).collect();
    let mut ret: Vec<f64> = Vec::new();
    let windows = Window::new(n, &series);

    for w in windows {
        ret.push(average(w));
    }

    Some(ret)
}

fn filter_nan(series: &[f64]) -> impl Iterator<Item = f64> + '_ {
    series
        .iter()
        .filter_map(|v| if v.is_nan() { None } else { Some(*v) })
}

// NOTE: Use when you're convinced that both x and y aren't NAN.
fn cmp_f64() -> Box<dyn FnMut(&f64, &f64) -> std::cmp::Ordering> {
    Box::new(|x: &f64, y: &f64| x.partial_cmp(y).unwrap())
}

fn average(series: &[f64]) -> f64 {
    if series.is_empty() {
        0.0
    } else {
        series.iter().sum::<f64>() / series.len() as f64
    }
}

struct Window<'a> {
    current_idx: usize,
    size: usize,
    series: &'a [f64],
}

impl<'a> Window<'a> {
    fn new(size: usize, series: &'a [f64]) -> Self {
        Self {
            current_idx: 0,
            size,
            series,
        }
    }
}

impl<'a> Iterator for Window<'a> {
    type Item = &'a [f64];

    fn next(&mut self) -> Option<&'a [f64]> {
        if self.current_idx + self.size > self.series.len() {
            None
        } else {
            let start = self.current_idx;
            let end = self.current_idx + self.size;

            self.current_idx += 1;
            Some(&self.series[start..end])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fn_min {
        use super::*;

        #[test]
        fn return_minimum_if_not_empty() {
            let series: Vec<f64> = vec![2.1, 1.3, 3.6];
            assert_eq!(Some(1.3), min(&series));
        }

        #[test]
        fn return_none_if_empty() {
            let series: Vec<f64> = vec![];
            assert_eq!(None, min(&series));
        }

        #[test]
        fn ignore_nan() {
            let series: Vec<f64> = vec![2.1, std::f64::NAN, 3.6];
            assert_eq!(Some(2.1), min(&series));
        }
    }

    mod fn_max {
        use super::*;

        #[test]
        fn return_maximum_if_not_empty() {
            let series: Vec<f64> = vec![2.1, 1.3, 3.6];
            assert_eq!(Some(3.6), max(&series));
        }

        #[test]
        fn return_none_if_empty() {
            let series: Vec<f64> = vec![];
            assert_eq!(None, max(&series));
        }

        #[test]
        fn ignore_nan() {
            let series: Vec<f64> = vec![2.1, std::f64::NAN, 3.6];
            assert_eq!(Some(3.6), max(&series));
        }
    }

    mod fn_n_window_sma {
        use super::*;

        #[test]
        fn return_simple_moving_averages() {
            let series: Vec<f64> = vec![2.1, 7.2, 3.6, 0.0];
            assert_eq!(Some(vec![4.3, 3.6]), n_window_sma(3, &series));
        }

        #[test]
        fn return_empty_if_given_empty() {
            let series: Vec<f64> = vec![];
            assert_eq!(Some(vec![]), n_window_sma(3, &series));
        }
    }

    mod fn_average {
        use super::*;

        #[test]
        fn return_average_if_not_empty() {
            let series: Vec<f64> = vec![2.1, 7.2, 3.6];
            assert_eq!(4.3, average(&series));
        }

        #[test]
        fn return_zero_if_empty() {
            let series: Vec<f64> = vec![];
            assert_eq!(0.0, average(&series));
        }
    }

    mod window_iter {
        use super::*;

        #[test]
        fn devide_into_iterator() {
            let series: Vec<f64> = vec![2.1, 1.3, 3.6, 7.2, 4.4];
            let mut window = Window::new(3, &series);
            assert_eq!(window.next(), Some(vec![2.1, 1.3, 3.6].as_slice()));
            assert_eq!(window.next(), Some(vec![1.3, 3.6, 7.2].as_slice()));
            assert_eq!(window.next(), Some(vec![3.6, 7.2, 4.4].as_slice()));
            assert_eq!(window.next(), None);
        }

        #[test]
        fn return_none_if_window_size_is_larger_than_slice_size() {
            let series: Vec<f64> = vec![2.1, 1.3];
            let mut window = Window::new(3, &series);
            assert_eq!(window.next(), None);
        }
    }
}
