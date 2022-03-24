use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::math::asin;

pub(super) fn azimuthal_raw<T>(p: &Coordinate<T>, scale: fn(T) -> T) -> Coordinate<T>
where
    T: CoordFloat,
{
    let cx = p.x.cos();
    let cy = p.y.cos();
    let k = scale(cx * cy);
    if k.is_infinite() {
        return Coordinate {
            x: T::from(2.0_f64).unwrap(),
            y: T::zero(),
        };
    }
    Coordinate {
        x: k * cy * p.x.sin(),
        y: k * p.y.sin(),
    }
}

pub(super) fn azimuthal_invert<T>(p: &Coordinate<T>, angle: fn(z: T) -> T) -> Coordinate<T>
where
    T: CoordFloat + FloatConst,
{
    let z = (p.x * p.x + p.y * p.y).sqrt();
    let c = angle(z);
    let sc = c.sin();
    let cc = c.cos();

    let ret_x = (p.x * sc).atan2(z * cc);
    let y_out: T =  if z == T::zero() {
        z
    } else {
         p.y * sc / z
    };
    let ret_y = asin(y_out);

    Coordinate { x: ret_x, y: ret_y }
}
