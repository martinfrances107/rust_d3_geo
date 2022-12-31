use geo::CoordFloat;

/// Generates a numeric sequence starting from the given start and stop values.
///
/// The output range does not include 'stop'.
#[allow(clippy::similar_names)]
pub fn range<T>(start: T, stop: T, step: T) -> Vec<T>
where
    T: CoordFloat,
{
    let count = ((stop - start) / step).floor().to_usize();
    count.map_or_else(
        || Vec::new(),
        |count| {
            let mut v = Vec::with_capacity(count);

            let mut value = start;
            while value < stop {
                v.push(value);
                value = value + step;
            }
            v
        },
    )
}

#[cfg(test)]
mod range_test {
    extern crate pretty_assertions;

    use crate::range::range;

    use pretty_assertions::assert_eq;

    #[test]
    fn returns_start_stop_step() {
        println!("range(start, stop, step) returns [start, start + step, start + 2 * step, … stop - step]");
        assert_eq!(
            range(0_f64, 5_f64, 1_f64),
            vec![0_f64, 1_f64, 2_f64, 3_f64, 4_f64]
        );
        assert_eq!(range(0_f64, 5_f64, 2_f64), vec![0_f64, 2_f64, 4_f64]);
        assert_eq!(range(2_f64, 5_f64, 2_f64), vec![2_f64, 4_f64]);
        assert_eq!(range(-1_f64, 3_f64, 2_f64), vec![-1_f64, 1_f64]);
    }

    #[test]
    fn returns_empty() {
        println!("range(start, stop, step) returns an empty array if start >= stop and step > 0");
        assert_eq!(range(5_f64, 5_f64, 2_f64), Vec::<f64>::new());
        assert_eq!(range(6_f64, 5_f64, 2_f64), Vec::<f64>::new());
        assert_eq!(range(10_f64, 10_f64, 1_f64), Vec::<f64>::new());
        assert_eq!(range(10_f64, 10_f64, 0.5_f64), Vec::<f64>::new());

        assert_eq!(range(0_f64, 0_f64, 1_f64), Vec::<f64>::new());
        assert_eq!(range(0_f64, 0_f64, 0.5_f64), Vec::<f64>::new());

        assert_eq!(range(20_f64, 10_f64, 2_f64), Vec::<f64>::new());
        assert_eq!(range(20_f64, 10_f64, 1_f64), Vec::<f64>::new());
        assert_eq!(range(20_f64, 10_f64, 0.5_f64), Vec::<f64>::new());
    }
}
