use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// use crate::math::epsilon;

use crate::stream::Stream;

pub fn interpolate<T: CoordFloat + FloatConst>(
    from: Option<Coordinate<T>>,
    to: Option<Coordinate<T>>,
    direction: T,
    stream: Box<dyn Stream<T>>,
) {
    let phi: T;
    let mut stream = stream;
    match from {
        None => {
            phi = direction * T::FRAC_PI_2();
            stream.point(-T::PI(), phi, None);
            stream.point(T::zero(), phi, None);
            stream.point(T::PI(), phi, None);
            stream.point(T::PI(), T::zero(), None);
            stream.point(T::PI(), -phi, None);
            stream.point(T::zero(), -phi, None);
            stream.point(-T::PI(), -phi, None);
            stream.point(-T::PI(), T::zero(), None);
            stream.point(-T::PI(), phi, None);
        }
        Some(from) => {
            // TODO investigate is to and Option<f64>
            let to = to.unwrap();
            if (from.x - to.x).abs() > T::epsilon() {
                let lambda = if from.x < to.x { T::PI() } else { -T::PI() };

                phi = direction * lambda / T::from(2).unwrap();
                stream.point(-lambda, phi, None);
                stream.point(T::zero(), phi, None);
                stream.point(lambda, phi, None);
            } else {
                stream.point(to.x, to.y, None);
            }
        }
    }
}
