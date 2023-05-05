// use std::fmt::Debug;

// use geo::CoordFloat;
// use geo_types::Coord;
// use num_traits::FloatConst;

// use crate::projection::ClipExtentAdjust;
// use crate::projection::ClipExtentSet;
// use crate::stream::Stream;

// use super::Builder;

// // Code Repeated 2^2 times.
// // Variantion over ClipAntimeridian/ClipCircle as Clip is rebuilt.
// // Varariantion over Resample/None as Resample is rebuilt.
// impl<DRAIN, T> ClipExtentAdjust for Builder<DRAIN, T>
// where
//     T: CoordFloat + Debug + Default + FloatConst,
//     DRAIN: Clone + Stream<EP = DRAIN, T = T>,
// {
//     type T = T;

//     #[inline]
//     fn clip_extent_adjust(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
//         self.pr.alaska.clip_extent_set(extent);
//         self
//     }
// }
