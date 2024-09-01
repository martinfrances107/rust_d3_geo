use geo::CoordFloat;

/// Generates a numeric sequence starting from the given start and stop values.
///
/// The output range does not include 'stop'.
#[allow(clippy::similar_names)]
pub fn range<T>(start: T, stop: T, step: T) -> Vec<T>
where
    T: CoordFloat,
{
    let count = ((stop - start) / step).ceil().to_usize();
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
    fn returns_one() {
        println!("range(stop) returns [0, 1, 2, … stop - 1]");
        assert_eq!(
            range(0_f64, 5_f64, 1_f64),
            vec![0_f64, 1_f64, 2_f64, 3_f64, 4_f64]
        );
        assert_eq!(range(0_f64, 2.01_f64, 1_f64), vec![0_f64, 1_f64, 2_f64]);
        assert_eq!(range(0_f64, 1_f64, 1_f64), vec![0_f64]);
        assert_eq!(range(0_f64, 0.5_f64, 1_f64), vec![0_f64]);
    }

    #[test]
    fn returns_empty_stop_less_than_or_equal_zero() {
        println!("range(stop) returns an empty array if stop <= 0");
        assert_eq!(range(0_f64, 0_f64, 1_f64), vec![]);
        assert_eq!(range(0_f64, -0.5_f64, 1_f64), vec![]);
        assert_eq!(range(0_f64, -1_f64, 1_f64), vec![]);
    }

    #[test]
    fn returns_empty_if_stop_is_nan() {
        println!("range(stop) returns an empty array if stop is NaN");
        assert_eq!(range(0_f64, f64::NAN, 1_f64), vec![]);
    }

    // it("range(start, stop) returns [start, start + 1, … stop - 1]", () => {
    //   assert.deepStrictEqual(range(0, 5), [0, 1, 2, 3, 4]);
    //   assert.deepStrictEqual(range(2, 5), [2, 3, 4]);
    //   assert.deepStrictEqual(range(2.5, 5), [2.5, 3.5, 4.5]);
    //   assert.deepStrictEqual(range(-1, 3), [-1, 0, 1, 2]);
    // });
    #[test]
    fn range_possible_js_semi_duplicate() {
        println!("range(start, stop) returns [start, start + 1, … stop - 1]");
        assert_eq!(
            range(0_f64, 5_f64, 1_f64),
            vec![0_f64, 1_f64, 2_f64, 3_f64, 4_f64]
        );
        assert_eq!(range(2_f64, 5_f64, 1_f64), vec![2_f64, 3_f64, 4_f64]);
        assert_eq!(
            range(2.5_f64, 5_f64, 1_f64),
            vec![2.5_f64, 3.5_f64, 4.5_f64]
        );
        assert_eq!(
            range(-1_f64, 3_f64, 1_f64),
            vec![-1_f64, 0_f64, 1_f64, 2_f64]
        );
    }

    #[test]
    fn empty_if_stop_or_stop_is_nan() {
        println!(
            "range(start, stop) returns an empty array if start or stop is NaN"
        );
        assert_eq!(range(0_f64, f64::NAN, 1_f64), vec![]);
        assert_eq!(range(1_f64, f64::NAN, 1_f64), vec![]);
        assert_eq!(range(-1_f64, f64::NAN, 1_f64), vec![]);
        assert_eq!(range(f64::NAN, f64::NAN, 1_f64), vec![]);
    }

    #[test]
    fn empty_if_start_greater_or_equal_stop() {
        println!("range(start, stop) returns an empty array if start >= stop");
        assert_eq!(range(0_f64, 0_f64, 1_f64), vec![]);
        assert_eq!(range(5_f64, 5_f64, 1_f64), vec![]);
        assert_eq!(range(6_f64, 5_f64, 1_f64), vec![]);
        assert_eq!(range(10_f64, 10_f64, 1_f64), vec![]);
        assert_eq!(range(20_f64, 10_f64, 1_f64), vec![]);
    }

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
