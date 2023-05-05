// use std::fmt::Debug;

// use geo::CoordFloat;
// use num_traits::FloatConst;

// use super::Builder;

// use crate::projection::ScaleSet;
// use crate::projection::TranslateGet;
// use crate::projection::TranslateSet;
// use crate::stream::Stream;

// impl<DRAIN, T> ScaleSet for Builder<DRAIN, T>
// where
//     T: CoordFloat + Debug + Default + FloatConst,
//     DRAIN: Clone + Stream<EP = DRAIN, T = T>,
// {
//     type T = T;

//     fn scale_set(&mut self, scale: T) -> &mut Self {
//         self.pr.alaska.scale_set(T::from(0.35_f64).unwrap() * scale);
//         self.pr.lower_48.scale_set(scale);
//         self.pr.hawaii.scale_set(scale);
//         self.translate_set(&self.pr.lower_48.translate())
//     }
// }
