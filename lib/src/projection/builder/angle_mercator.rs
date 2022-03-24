// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use num_traits::FloatConst;

// use crate::clip::Interpolate;
// use crate::clip::Line;
// use crate::clip::PointVisible;
// use crate::projection::resampler::none::None as ResampleNone;
// use crate::projection::resampler::resample::Resample;
// use crate::projection::ProjectionRawBase;
// use crate::projection::ProjectionRawMercator;
// use crate::stream::Connected;
// use crate::stream::Stream;
// use crate::stream::Unconnected;

// use super::Angle;
// use super::Builder;

// impl<DRAIN, INTERPOLATE, LINE, PCN, PR, PV, T> Angle
//     for Builder<DRAIN, INTERPOLATE, LINE, PCN, PR, PV, Resample<DRAIN, PR, PCN, Unconnected, T>, T>
// where
//     INTERPOLATE: Interpolate<T = T>,
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     LB: Line, LC: Line, LU: Line,
//     PR: ProjectionRawMercator<T>,
//     PV: PointVisible<T = T>,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     /// f64 or f32.
//     type T = T;

//     /// Returns the projection’s post-projection planar rotation angle.
//     /// defaults to 0°.
//     #[inline]
//     fn get_angle(&self) -> Self::T {
//         self.base().alpha.to_degrees()
//     }

//     /// Sets the projection’s post-projection planar rotation angle to the
//     /// specified angle in degrees and returns the projection.
//     ///
//     fn angle(mut self, angle: T) -> Self {
//         self.base().alpha = (angle % T::from(360_f64).unwrap()).to_radians();
//         self.recenter_with_resampling()
//     }
// }

// impl<DRAIN, INTERPOLATE, LINE, PCN, PR, PV, T> Angle
//     for Builder<
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
//     PR: ProjectionRawMercator<T>,
//     PV: PointVisible<T = T>,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     /// f64 or f32.
//     type T = T;

//     /// Returns the projection’s post-projection planar rotation angle.
//     /// defaults to 0°.
//     #[inline]
//     fn get_angle(&self) -> Self::T {
//         self.base().alpha.to_degrees()
//     }

//     /// Sets the projection’s post-projection planar rotation angle to the
//     /// specified angle in degrees and returns the projection.
//     ///
//     fn angle(mut self, angle: T) -> Self {
//         self.base().alpha = (angle % T::from(360_f64).unwrap()).to_radians();
//         self.recenter_no_resampling()
//     }
// }
