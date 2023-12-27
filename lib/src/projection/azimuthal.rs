use geo::CoordFloat;
use geo_types::Coord;

pub(super) fn azimuthal_raw<T>(p: &Coord<T>, scale: fn(T) -> T) -> Coord<T>
where
    T: CoordFloat,
{
    let (sx, cx) = p.x.sin_cos();
    let (sy, cy) = p.y.sin_cos();
    let k = scale(cx * cy);
    if k.is_infinite() {
        return Coord {
            x: T::from(2.0_f64).unwrap(),
            y: T::zero(),
        };
    }
    Coord {
        x: k * cy * sx,
        y: k * sy,
    }
}

pub(super) fn azimuthal_invert<T>(
    p: &Coord<T>,
    angle: fn(z: T) -> T,
) -> Coord<T>
where
    T: CoordFloat,
{
    let z = (p.x * p.x + p.y * p.y).sqrt();
    let c = angle(z);
    let (sc, cc) = c.sin_cos();
    let ret_x = (p.x * sc).atan2(z * cc);
    let y_out: T = if z.is_zero() { z } else { p.y * sc / z };
    let ret_y = y_out.asin();

    Coord { x: ret_x, y: ret_y }
}
