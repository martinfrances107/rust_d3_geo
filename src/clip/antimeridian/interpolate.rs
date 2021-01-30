// use crate::stream::StreamNode;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::FloatConst;

// pub fn interpolate<T: CoordFloat + FloatConst>(
//     from: Option<Coordinate<T>>,
//     to: Option<Coordinate<T>>,
//     direction: T,
//     stream: StreamNode<T>,
// ) {
//     let phi: T;
//     let mut s = stream.borrow_mut();
//     match from {
//         None => {
//             phi = direction * T::FRAC_PI_2();
//             s.point(-T::PI(), phi, None);
//             s.point(T::zero(), phi, None);
//             s.point(T::PI(), phi, None);
//             s.point(T::PI(), T::zero(), None);
//             s.point(T::PI(), -phi, None);
//             s.point(T::zero(), -phi, None);
//             s.point(-T::PI(), -phi, None);
//             s.point(-T::PI(), T::zero(), None);
//             s.point(-T::PI(), phi, None);
//         }
//         Some(from) => {
//             // TODO investigate is to and Option<f64>
//             // let mut s = stream.borrow_mut();
//             let to = to.unwrap();
//             if (from.x - to.x).abs() > T::epsilon() {
//                 let lambda = if from.x < to.x { T::PI() } else { -T::PI() };

//                 phi = direction * lambda / T::from(2).unwrap();
//                 s.point(-lambda, phi, None);
//                 s.point(T::zero(), phi, None);
//                 s.point(lambda, phi, None);
//             } else {
//                 s.point(to.x, to.y, None);
//             }
//         }
//     }
// }
