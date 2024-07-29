/// Generates great circles.
pub mod builder;

use geo::CoordFloat;
use geo::Geometry;
use geo::MultiLineString;
use geo_types::Coord;

use crate::math::EPSILON;
use crate::range::range;

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

/// Helper function returns the default graticule builder
///
/// # Panics
/// `unwrap()` is used here but a panic will never happen as -180,-90,-80,+80,+90,+180 will always be converted into T.
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

/// Helper function returns the default graticule.
///
/// In javascript rendering to a canvas context  will look
/// like to example below.
///
/// ```javascript
///  let graticule = d3.geoGraticule();
///    context.beginPath();
///    context.strokeStyle = '#ccc';
///    geoGenerator(graticule());
///    context.stroke();
/// ```
///
/// Here is the equivalent rust version.
///```rustlang
///  // Graticule RUSTLANG
///  let graticule = generate_mls();
///  context_raw.begin_path();
///  context_raw.set_stroke_style(&"#ccc".into());
///  path.object(&graticule);
///  context_raw.stroke();
/// ```
#[must_use]
pub fn generate_mls<T>() -> Geometry<T>
where T: 'static + CoordFloat{
    Geometry::MultiLineString(MultiLineString(generate::<T>().lines().collect()))
}
