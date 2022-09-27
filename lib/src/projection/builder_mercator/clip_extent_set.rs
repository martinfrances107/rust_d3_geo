// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::FloatConst;

// use crate::projection::builder::template::ClipU;
// use crate::projection::builder::template::NoClipU;
// use crate::projection::builder::template::ResampleClipU;
// use crate::projection::builder::template::ResampleNoneClipC;
// use crate::projection::builder::template::ResampleNoneClipU;
// use crate::projection::builder::template::ResampleNoneNoClipU;
// use crate::projection::ClipExtentSet;
// use crate::projection::TransformExtent;
// use crate::stream::Stream;
// use crate::Transform;

// use super::Builder;
// use super::ReclipConvert;
// use super::ResampleNoClipU;

// // TOD must vary by ClipAntimeridian -- 2 more impl blocks

// impl<CLIP, DRAIN, PR, T> ClipExtentSet
//     for Builder<CLIP, DRAIN, NoClipU<DRAIN>, PR, ResampleNoClipU<DRAIN, PR, T>, T>
// where
//     DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
//     PR: Clone + Transform<T = T> + TransformExtent<T = T>,
//     T: CoordFloat + Default + FloatConst,
// {
//     type Output = Builder<CLIP, DRAIN, ClipU<DRAIN, T>, PR, ResampleClipU<DRAIN, PR, T>, T>;
//     type T = T;

//     fn clip_extent_set(mut self, extent: &[Coordinate<T>; 2]) -> Self::Output {
//         self.extent = Some(*extent);
//         self.reclip_convert()
//     }
// }

// impl<CLIP, DRAIN, PR, T> ClipExtentSet
//     for Builder<CLIP, DRAIN, NoClipU<DRAIN>, PR, ResampleNoClipU<DRAIN, PR, T>, T>
// where
//     DRAIN: Clone,
//     PR: Clone + Transform<T = T>,
//     T: CoordFloat + FloatConst,
// {
//     type Output = Builder<CLIP, DRAIN, ClipU<DRAIN, T>, PR, ResampleClipU<DRAIN, PR, T>, T>;
//     type T = T;

//     #[inline]
//     fn clip_extent_set(self, extent: &[Coordinate<T>; 2]) -> Self::Output {
//         // Architecture Discussion:
//         // CLIP is generic over <.. RC, RU,..>,
//         // So a change in the resample type causes rebuilding of clip.
//         Self::Output {
//             base: self.base.clip_extent_set(extent),
//             pr: self.pr,
//             // Mutate section.
//             extent: Some(*extent),
//         }
//     }
// }

// impl<CLIP, DRAIN, PR, T> ClipExtentSet
//     for Builder<CLIP, DRAIN, NoClipU<DRAIN>, PR, ResampleNoneNoClipU<DRAIN, PR, T>, T>
// where
//     DRAIN: Clone,
//     PR: Clone,
//     T: CoordFloat + FloatConst,
// {
//     type Output = Builder<
//         CLIP,
//         DRAIN,
//         ClipU<DRAIN, T>,
//         ResampleNoneClipC<DRAIN, PR, T>,
//         ResampleNoneClipU<DRAIN, PR, T>,
//         T,
//     >;
//     type T = T;

//     #[inline]
//     fn clip_extent_set(self, extent: &[Coordinate<T>; 2]) -> Self::Output {
//         // Architecture Discussion:
//         // CLIP is generic over <.. RC, RU,..>,
//         // So a change in the resample type causes rebuilding of clip.
//         Self::Output {
//             base: self.base.clip_extent_set(extent),
//             pr: self.pr,
//             // Mutate section.
//             extent: Some(*extent),
//         }
//     }
// }
