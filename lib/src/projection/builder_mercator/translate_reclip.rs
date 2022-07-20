// use std::fmt::Debug;

// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::FloatConst;

// use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
// use crate::projection::builder_mercator::BuilderMercatorAntimeridianResampleNoClip;
// use crate::projection::TransformExtent;
// use crate::stream::Stream;
// use crate::Transform;

// use super::ReclipAdjust;
// use super::TranslateReclip;

// impl<DRAIN, PR, T> TranslateReclip for BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T>
// where
//     DRAIN: Clone + Default + Debug + Stream<EP = DRAIN, T = T>,
//     PR: Clone + Transform<T = T> + TransformExtent<T = T>,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type Output = BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>;
//     type T = T;

//     #[inline]
//     fn translate(mut self, t: &Coordinate<T>) -> Self::Output {
//         self.base.x = t.x;
//         self.base.y = t.y;
//         self.reclip()
//     }
// }
