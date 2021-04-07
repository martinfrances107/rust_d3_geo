// use geo::CoordFloat;
// use num_traits::FloatConst;

// use super::rejoin::intersection::Intersection;

// pub fn compare_intersection<T>(a: Intersection<T>, b: Intersection<T>) -> T
// where
//     T: CoordFloat + FloatConst,
// {
//     // let a_dashed = a.x;

//     match (a.x, b.x) {
//         (Some(ax), Some(bx)) => {
//             let part1 = match ax.p.x < T::zero() {
//                 true => ax.p.y - T::FRAC_PI_2() - T::epsilon(),
//                 false => T::FRAC_PI_2() - ax.p.y,
//             };
//             let part2 = match bx.p.x < T::zero() {
//                 true => bx.p.y - T::FRAC_PI_2() - T::epsilon(),
//                 false => T::FRAC_PI_2() - bx.p.y,
//             };
//             return part1 - part2;
//         }
//         _ => {
//             panic!("compare call with Empty inputs.")
//         }
//     }
// }
