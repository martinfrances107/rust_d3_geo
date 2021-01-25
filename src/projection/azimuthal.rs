use geo::{CoordFloat, Coordinate, Point};

pub fn azimuthal_raw<T: CoordFloat + 'static>(
    scale: Box<dyn Fn(T) -> T>,
) -> Box<dyn Fn(T, T) -> Point<T>> {
    return Box::new(move |x: T, y: T| -> Point<T> {
        let cx = x.cos();
        let cy = y.cos();
        let k = scale(cx * cy);
        return match k.is_infinite() {
            true => Point::new(T::from(2).unwrap(), T::zero()),
            false => Point::new(k * cy * x.sin(), k * y.sin()),
        };
    });
}

pub fn azimuthal_invert<T: CoordFloat + 'static>(
    angle: Box<dyn Fn(T) -> T>,
) -> Box<dyn Fn(T, T) -> Coordinate<T>> {
    return Box::new(move |x: T, y: T| -> Coordinate<T> {
        let z = (x * x + y * y).sqrt();
        let c = angle(z);
        let sc = c.sin();
        let cc = c.cos();

        let ret_x = (x * sc).atan2(z * cc);
        let y_out;
        if z == T::zero() {
            y_out = z;
        } else {
            y_out = y * sc / z;
        }
        let ret_y = y_out.asin();

        return Coordinate { x: ret_x, y: ret_y };
    });
}
