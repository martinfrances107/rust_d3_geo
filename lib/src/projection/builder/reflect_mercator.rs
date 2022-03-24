// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use crate::clip::Interpolate;
// use crate::clip::Line;
// use crate::clip::PointVisible;
// // use crate::projection::mercator_builder::MercatorBuilder;
// use crate::projection::resampler::none::None as ResampleNone;
// use crate::projection::resampler::resample::Resample;
// use crate::projection::ProjectionRawBase;
// use crate::projection::Reflect;
// use crate::stream::Connected;
// use crate::stream::Stream;
// use crate::stream::Unconnected;

// use super::Builder;

// impl<DRAIN, INTERPOLATE, LINE, PCN, PR, PV, T> Reflect
//     for MercatorBuilder<
//         DRAIN,
//         INTERPOLATE,
//         LINE,
//         PCN,
//         PR,
//         PV,
//         Resample<DRAIN, PR, PCN, Unconnected, T>,
//         T,
//     >
// where
//     INTERPOLATE: Interpolate<T = T>,
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     LB: Line, LC: Line, LU: Line,
//     PR: ProjectionRawBase<T>,
//     PV: PointVisible<T = T>,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type T = T;

//     /// Is the projection builder set to invert the x-coordinate.
//     #[inline]
//     fn get_reflect_x(&self) -> bool {
//         self.base.sx < T::zero()
//     }

//     /// Set the projection builder to invert the x-coordinate.
//     fn reflect_x(mut self, reflect: bool) -> Self {
//         if reflect {
//             self.base.sx = T::from(-1.0_f64).unwrap();
//         } else {
//             self.base.sx = T::one();
//         }
//         self.base.recenter_with_resampling()
//     }

//     /// Is the projection builder set to invert the y-coordinate.
//     #[inline]
//     fn get_reflect_y(&self) -> bool {
//         self.base.sy < T::zero()
//     }

//     /// Set the projection builder to invert the y-coordinate.
//     #[inline]
//     fn reflect_y(mut self, reflect: bool) -> Self {
//         if reflect {
//             self.base.sy = T::from(-1.0_f64).unwrap();
//         } else {
//             self.base.sy = T::one();
//         }
//         self.base.recenter_with_resampling()
//     }
// }

// impl<DRAIN, INTERPOLATE, LINE, PCN, PR, PV, T> Reflect
//     for MercatorBuilder<
//         DRAIN,
//         INTERPOLATE,
//         LINE,
//         PCN,
//         PR,
//         PV,
//         ResampleNone<DRAIN, PR, PCN, Unconnected, T>,
//         T,
//     >
// where
//     INTERPOLATE: Interpolate<T = T>,
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     LB: Line, LC: Line, LU: Line,
//     PR: ProjectionRawBase<T>,
//     PV: PointVisible<T = T>,
//     T: 'static + AsPrimitive<T> + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type T = T;

//     /// Is the projection builder set to invert the x-coordinate.
//     #[inline]
//     fn get_reflect_x(&self) -> bool {
//         self.base.sx < T::zero()
//     }

//     /// Set the projection builder to invert the x-coordinate.
//     fn reflect_x(mut self, reflect: bool) -> Self {
//         if reflect {
//             self.base.sx = T::from(-1.0_f64).unwrap();
//         } else {
//             self.base.sx = T::one();
//         }
//         self.base.recenter_no_resampling()
//     }

//     /// Is the projection builder set to invert the y-coordinate.
//     #[inline]
//     fn get_reflect_y(&self) -> bool {
//         self.base.sy < T::zero()
//     }

//     /// Set the projection builder to invert the y-coordinate.
//     #[inline]
//     fn reflect_y(mut self, reflect: bool) -> Self {
//         if reflect {
//             self.base.sy = T::from(-1.0_f64).unwrap();
//         } else {
//             self.base.sy = T::one();
//         }
//         self.base.recenter_no_resampling()
//     }
// }
