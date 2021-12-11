use geo::{CoordFloat, Coordinate};

pub(super) fn azimuthal_raw<T>(p: &Coordinate<T>, scale: fn(T) -> T) -> Coordinate<T>
where
    T: CoordFloat,
{
    let cx = p.x.cos();
    let cy = p.y.cos();
    let k = scale(cx * cy);
    if k.is_infinite() {
        return Coordinate {
            x: T::from(2.0).unwrap(),
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
    T: CoordFloat,
{
    let z = (p.x * p.x + p.y * p.y).sqrt();
    let c = angle(z);
    let sc = c.sin();
    let cc = c.cos();

    let ret_x = (p.x * sc).atan2(z * cc);
    let y_out;
    if z == T::zero() {
        y_out = z;
    } else {
        y_out = p.y * sc / z;
    }
    let ret_y = y_out.asin();

    Coordinate { x: ret_x, y: ret_y }
}
