// use std::fmt::Debug;

// use geo::CoordFloat;
// use geo_types::Coord;
// use num_traits::FloatConst;

// use crate::math::EPSILON;
// use crate::projection::ClipExtentSet;
// use crate::projection::ScaleGet;
// use crate::projection::TranslateSet;

// use super::Builder;

// impl<DRAIN, T> TranslateSet for Builder<DRAIN, T>
// where
//     DRAIN: Clone,
//     T: CoordFloat +  Default + FloatConst,
// {
//     type T = T;

//     fn translate_set(&mut self, t: &Coord<T>) -> &mut Self {
//         let epsilon = T::from(EPSILON).unwrap();
//         let k: T = self.pr.lower_48.scale();

//         self.pr.lower_48_point = self.pr.lower_48.translate_set(t).clip_extent_set(&[
//             Coord {
//                 x: T::from(0.455_f64).unwrap().mul_add(-k, t.x),
//                 y: T::from(0.234f64).unwrap().mul_add(-k, t.y),
//             },
//             Coord {
//                 x: T::from(0.455_f64).unwrap().mul_add(k, t.x),
//                 y: T::from(0.234f64).unwrap().mul_add(k, t.y),
//             },
//         ]);

//         self.pr.alaska_point = self
//             .pr
//             .alaska
//             .translate_set(&Coord {
//                 x: T::from(0.307_f64).unwrap().mul_add(-k, t.x),
//                 y: T::from(0.201f64).unwrap().mul_add(-k, t.y),
//             })
//             .clip_extent_set(&[
//                 Coord {
//                     x: T::from(0.425_f64).unwrap().mul_add(-k, t.x) + epsilon,
//                     y: T::from(0.120f64).unwrap().mul_add(-k, t.y) + epsilon,
//                 },
//                 Coord {
//                     x: T::from(0.214_f64).unwrap().mul_add(-k, t.x) - epsilon,
//                     y: T::from(0.234f64).unwrap().mul_add(-k, t.y) - epsilon,
//                 },
//             ]);

//         self.pr.hawaii = self
//             .pr
//             .hawaii
//             .translate_set(&Coord {
//                 x: T::from(0.205_f64).unwrap().mul_add(-k, t.x),
//                 y: T::from(0.212f64).unwrap().mul_add(-k, t.y),
//             })
//             .clip_extent_set(&[
//                 Coord {
//                     x: T::from(0.214_f64).unwrap().mul_add(-k, t.x) + epsilon,
//                     y: T::from(0.166f64).unwrap().mul_add(-k, t.y) + epsilon,
//                 },
//                 Coord {
//                     x: T::from(0.214f64).unwrap().mul_add(-k, t.x) + epsilon,
//                     y: T::from(0.234f64).unwrap().mul_add(k, t.y) - epsilon,
//                 },
//             ]);
//         self
//     }
// }
