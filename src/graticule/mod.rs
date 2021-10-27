pub mod graticule;

use crate::math::EPSILON;
use geo::CoordFloat;
use graticule::Graticule;

// TODO: Code smell. I needed something of the form 0_f64..100_f64.step_by(0.1)
// but I had to hand craft this!
fn range<T>(start: T, stop: T, step: T) -> Vec<T>
where
    T: CoordFloat,
{
    let mut v = Vec::new();
    let mut value = start;
    // JS has concat which I don;t understand.
    while value < stop {
        v.push(value);
        value = value + step;
    }
    v
}

type CoordFn<T> = Box<dyn Fn(T) -> Vec<(T, T)>>;

fn graticule_x<T>(y0: T, y1: T, dy: T) -> CoordFn<T>
where
    T: 'static + CoordFloat,
{
    let y = range(y0, y1 - T::from(EPSILON).unwrap(), dy);
    Box::new(move |x| y.iter().map(|y| (x, *y)).collect())
}

fn graticule_y<T>(x0: T, x1: T, dx: T) -> CoordFn<T>
where
    T: 'static + CoordFloat,
{
    let x = range(x0, x1 - T::from(EPSILON).unwrap(), dx);
    Box::new(move |y| x.iter().map(|x| (*x, y)).collect())
}

/// Returns the default graticule.
pub fn generate<T>() -> Graticule<T>
where
    T: 'static + CoordFloat,
{
    Graticule::default()
        .extent_major([
            [T::from(-180).unwrap(), T::from(-90_f64 + EPSILON).unwrap()],
            [
                T::from(180_f64).unwrap(),
                T::from(90_f64 - EPSILON).unwrap(),
            ],
        ])
        .extent_minor([
            [T::from(-180).unwrap(), T::from(-80_f64 - EPSILON).unwrap()],
            [
                T::from(180_f64).unwrap(),
                T::from(80_f64 + EPSILON).unwrap(),
            ],
        ])
}
