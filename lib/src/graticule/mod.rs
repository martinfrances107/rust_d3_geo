/// Generates great circles.
pub mod builder;

use geo::CoordFloat;
use geo_types::Coord;

use rust_d3_array::range::range;

use crate::math::EPSILON;

use builder::Builder as GraticuleBuilder;

type CoordFn<T> = Box<dyn Fn(T) -> Vec<Coord<T>>>;

fn graticule_x<T>(y0: T, y1: T, dy: T) -> CoordFn<T>
where
    T: 'static + CoordFloat,
{
    let mut y = range(y0, y1 - T::from(EPSILON).unwrap(), dy);
    y.push(y1);

    Box::new(move |x| y.iter().map(|y| Coord { x, y: *y }).collect())
}

fn graticule_y<T>(x0: T, x1: T, dx: T) -> CoordFn<T>
where
    T: 'static + CoordFloat,
{
    let mut x = range(x0, x1 - T::from(EPSILON).unwrap(), dx);
    x.push(x1);
    Box::new(move |y| x.iter().map(|x| Coord { x: *x, y }).collect())
}

/// Helper function returns the default graticule.
///
/// # Panics
///  Will never happen as +/-90 will always be converted into T.
#[must_use]
pub fn generate<T>() -> GraticuleBuilder<T>
where
    T: 'static + CoordFloat,
{
    let epsilon = T::from(EPSILON).unwrap();

    let mut out = GraticuleBuilder::default();
    out.extent_major_set([
        [T::from(-180).unwrap(), T::from(-90_f64).unwrap() + epsilon],
        [
            T::from(180_f64).unwrap(),
            T::from(90_f64).unwrap() - epsilon,
        ],
    ])
    .extent_minor_set([
        [T::from(-180).unwrap(), T::from(-80_f64).unwrap() - epsilon],
        [
            T::from(180_f64).unwrap(),
            T::from(80_f64).unwrap() + epsilon,
        ],
    ]);

    out
}
