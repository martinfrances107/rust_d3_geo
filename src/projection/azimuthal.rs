// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::FloatConst;

// #[inline]
// pub fn azimuthal_raw<'a, T: CoordFloat + FloatConst>(
//     scale: Box<dyn Fn(T) -> T>,
// ) -> Box<dyn Fn(&Coordinate<T>) -> Coordinate<T>> {
//     Box::new(move |p: &Coordinate<T>| -> Coordinate<T> {
//         let cx = p.x.cos();
//         let cy = p.y.cos();
//         let k = scale(cx * cy);
//         match k.is_infinite() {
//             true => Coordinate {
//                 x: T::from(2).unwrap(),
//                 y: T::zero(),
//             },
//             false => Coordinate {
//                 x: k * cy * p.x.sin(),
//                 y: k * p.y.sin(),
//             },
//         }
//     })
// }

// #[inline]
// pub fn azimuthal_invert<'a, T: CoordFloat + Default>(
//     angle: fn(T) -> T,
// ) -> Box<dyn Fn(&Coordinate<T>) -> Coordinate<T>> {
//     Box::new(move |p: &Coordinate<T>| -> Coordinate<T> {
//         let z = (p.x * p.x + p.y * p.y).sqrt();
//         let c = angle(z);
//         let sc = c.sin();
//         let cc = c.cos();

//         let ret_x = (p.x * sc).atan2(z * cc);
//         let y_out;
//         if z == T::zero() {
//             y_out = z;
//         } else {
//             y_out = p.y * sc / z;
//         }
//         let ret_y = y_out.asin();

//         Coordinate { x: ret_x, y: ret_y }
//     })
// }
