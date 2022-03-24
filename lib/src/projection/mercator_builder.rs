// use std::fmt::Display;

// use approx::AbsDiffEq;
// use derivative::*;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use crate::clip::antimeridian::gen_clip_antimeridian;
// use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
// use crate::clip::antimeridian::line::Line as LineAntimeridian;
// use crate::clip::antimeridian::pv::PV as PVAntimeridian;
// use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
// use crate::clip::circle::line::Line as LineCircle;
// use crate::clip::circle::pv::PV as PVCircle;
// use crate::clip::Interpolate;
// use crate::clip::Line;
// use crate::clip::PointVisible;
// use crate::identity::Identity;

// use crate::clip::rectangle::Rectangle;
// use crate::projection::builder::PostClipNode;
// use crate::rot::rotate_radians;
// use crate::stream::Connected;
// use crate::stream::Stream;
// use crate::stream::Streamable;
// use crate::stream::Unconnected;
// use crate::Transform;

// use super::builder::Builder as ProjectionBuilder;
// use super::resampler::resample::Resample;
// use super::resampler::Resampler;
// use super::stream_transform_radians::StreamTransformRadians;
// use super::Angle;
// use super::Bounds;
// use super::Center;
// use super::ClipAngle;
// use super::ClipExtent;
// use super::Fit;
// use super::Precision;
// use super::Projector;
// use super::ProjectionRawBase;
// use super::Reflect;
// use super::Scale;
// use super::TransformExtent;
// use super::Translate;
// use crate::projection::Rotate;

// /// A wrapper for Projection\Builder which overrides the traits - scale translate and center.
// #[derive(Clone, Derivative)]
// #[derivative(Debug)]
// pub struct MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     //     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     //     INTERPOLATE: Interpolate<T = T>,
//     //     LB: Line, LC: Line, LU: Line,
//     //     PV: PointVisible<T = T>,
//     //     PR: ProjectionRawBase<T>, // TODO limit this to only certain types of PR
//     //     RC: Resampler, RU: Resampler,
//     T: CoordFloat + FloatConst,
// {
//     pr: PR,
//     pub base: ProjectionBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>,
//     x0: Option<T>,
//     y0: Option<T>,
//     x1: Option<T>,
//     y1: Option<T>, // post-clip extent
// }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
//     MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     T: CoordFloat + FloatConst,
// {
//     #[inline]
//     pub fn base(
//         &mut self,
//     ) -> &mut ProjectionBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> {
//         &mut self.base
//     }
// }

// impl<DRAIN, PR, RESAMPLER, T>
//     MercatorBuilder<
//         DRAIN,
//         InterpolateAntimeridian<T>,
//         LineAntimeridian<DRAIN, RESAMPLER, Unconnected, T>,
//         Identity<DRAIN, DRAIN, Unconnected, T>,
//         PR,
//         PVAntimeridian<T>,
//         RESAMPLER,
//         T,
//     >
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     PR: ProjectionRawBase<T>,
//     RC: Resampler, RU: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     /// Wrap a default projector and provides mercator specific overrides.
//     pub fn new(pr: PR) -> Self {
//         let base: ProjectionBuilder<
//             DRAIN,
//             InterpolateAntimeridian<T>,
//             LineAntimeridian<DRAIN, RESAMPLER, Unconnected, T>,
//             Identity<DRAIN, DRAIN, Unconnected, T>,
//             PR,
//             PVAntimeridian<T>,
//             RESAMPLER,
//             T,
//         > = ProjectionBuilder::new(
//             gen_clip_antimeridian::<DRAIN, RESAMPLER, PR, T>(),
//             pr.clone(),
//         );
//         Self {
//             pr,
//             base,
//             x0: None,
//             y0: None,
//             x1: None,
//             y1: None,
//         }
//     }

//     /// Using the currently programmed state output a new projection.
//     #[inline]
//     pub fn build(
//         &self,
//     ) -> Projector<
//         DRAIN,
//         InterpolateAntimeridian<T>,
//         LineAntimeridian<DRAIN, RESAMPLER, Unconnected, T>,
//         Identity<DRAIN, DRAIN, Unconnected, T>,
//         PR,
//         PVAntimeridian<T>,
//         RESAMPLER,
//         T,
//     > {
//         Projector {
//             cache: None,
//             postclip: self.base.postclip,
//             clip: self.base.preclip,
//             resample: self.base,

//             rotate_transform: self.base.project_rotate_transform,
//             rotate_transform: self.base.rotate_transform,
//             rotator: self.base.rotate,
//             transform_radians: StreamTransformRadians(Unconnected),
//         }
//     }
// }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
//     MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     INTERPOLATE: Interpolate<T = T>,
//     LB: Line, LC: Line, LU: Line,
//     PR: TransformExtent<T>,
//     PV: PointVisible<T = T>,
//     RC: Resampler, RU: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     fn reclip(mut self) -> Self {
//         let k = T::PI() * self.get_scale();

//         let rotate_raw = self.base.get_rotate();
//         let t = rotate_radians(rotate_raw).invert(&Coordinate {
//             x: T::zero(),
//             y: T::zero(),
//         });
//         let t = self.base.build().transform(&t);
//         let ce = match (self.x0, self.y0, self.x1, self.y1) {
//             (Some(x0), Some(y0), Some(x1), Some(y1)) => {
//                 // MercatorRaw and MercatorTransverseRaw supply different
//                 // transforms
//                 self.pr.clone().transform_extent(k, t, x0, y0, x1, y1)
//             }
//             _ => [
//                 Coordinate {
//                     x: t.x - k,
//                     y: t.y - k,
//                 },
//                 Coordinate {
//                     x: t.x + k,
//                     y: t.y + k,
//                 },
//             ],
//         };

//         self.base = self.base.clip_extent(&ce);
//         self
//     }
// }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Center
//     for MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     INTERPOLATE: Interpolate<T = T>,
//     LB: Line, LC: Line, LU: Line,

//     PR: TransformExtent<T>,
//     PV: PointVisible<T = T>,
//     RC: Resampler, RU: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     type T = T;

//     #[inline]
//     fn get_center(&self) -> Coordinate<T> {
//         self.base.get_center()
//     }

//     fn center(mut self, center: &Coordinate<T>) -> Self {
//         self.base = self.base.center(center);
//         self.reclip()
//     }
// }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Scale
//     for MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     INTERPOLATE: Interpolate<T = T>,
//     LB: Line, LC: Line, LU: Line,
//     PR: TransformExtent<T>,
//     PV: PointVisible<T = T>,
//     RC: Resampler, RU: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     type T = T;

//     #[inline]
//     fn get_scale(&self) -> T {
//         self.base.get_scale()
//     }

//     fn scale(mut self, scale: T) -> Self {
//         self.base = self.base.scale(scale);
//         self.reclip()
//     }
// }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Translate
//     for MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     INTERPOLATE: Interpolate<T = T>,
//     LB: Line, LC: Line, LU: Line,
//     PR: TransformExtent<T>,
//     PV: PointVisible<T = T>,
//     RC: Resampler, RU: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     type T = T;

//     #[inline]
//     fn get_translate(&self) -> Coordinate<T> {
//         self.base.get_translate()
//     }

//     fn translate(mut self, t: &Coordinate<T>) -> Self {
//         self.base = self.base.translate(t);
//         self.reclip()
//     }
// }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Precision
//     for MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     INTERPOLATE: Interpolate<T = T>,
//     LB: Line, LC: Line, LU: Line,
//     PR: ProjectionRawBase<T>,
//     PV: PointVisible<T = T>,
//     RC: Resampler, RU: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     type Output = MercatorBuilder<
//         DRAIN,
//         INTERPOLATE,
//         LINE,
//         PCN,
//         PR,
//         PV,
//         Resample<DRAIN, PR, PCN, Unconnected, T>,
//         T,
//     >;
//     type T = T;
//     #[inline]
//     fn get_precision(&self) -> T {
//         self.base.get_precision()
//     }

//     fn precision(self, delta: &T) -> Self::Output {
//         self.base = self.base.precision(delta);
//         self
//     }
// }

// impl<I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Fit
//     for MercatorBuilder<Bounds<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     INTERPOLATE: Interpolate<T = T>,
//     LB: Line, LC: Line, LU: Line,
//     PR: ProjectionRawBase<T>,
//     PV: PointVisible<T = T>,
//     RC: Resampler, RU: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     type T = T;

//     #[inline]
//     fn fit_extent(mut self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
//     where
//         Self::T: AsPrimitive<T> + CoordFloat,
//     {
//         self.base = self.base.fit_extent(extent, object);
//         self
//     }

//     #[inline]
//     fn fit_size(mut self, size: [T; 2], object: &impl Streamable<T = T>) -> Self
//     where
//         Self::T: AsPrimitive<T> + CoordFloat,
//     {
//         self.base = self.base.fit_size(size, object);
//         self
//     }

//     #[inline]
//     fn fit_width(mut self, w: T, object: &impl Streamable<T = T>) -> Self
//     where
//         Self::T: AsPrimitive<T> + CoordFloat,
//     {
//         self.base = self.base.fit_width(w, object);
//         self
//     }

//     /// Similar to fit_size where the width is automatically chosen from
//     /// the aspect ratio of object and the given constraint on height.
//     #[inline]
//     fn fit_height(mut self, h: T, object: &impl Streamable<T = T>) -> Self
//     where
//         Self::T: AsPrimitive<T> + CoordFloat,
//     {
//         self.base = self.base.fit_height(h, object);
//         self
//     }
// }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> ClipExtent
//     for MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     INTERPOLATE: Interpolate<T = T>,
//     LB: Line, LC: Line, LU: Line,
//     PR: TransformExtent<T>,
//     PV: PointVisible<T = T>,
//     RC: Resampler, RU: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     /// f64 or f32.
//     type T = T;
//     type OutputClear = MercatorBuilder<
//         DRAIN,
//         INTERPOLATE,
//         LINE,
//         Identity<DRAIN, DRAIN, Unconnected, T>,
//         PR,
//         PV,
//         RESAMPLER,
//         T,
//     >;
//     type OutputBounded = MercatorBuilder<
//         DRAIN,
//         INTERPOLATE,
//         LINE,
//         Rectangle<DRAIN, DRAIN, Unconnected, T>,
//         PR,
//         PV,
//         RESAMPLER,
//         T,
//     >;
//     /// Returns a bounding box.
//     fn get_clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]> {
//         match (self.x0, self.y0, self.x1, self.y1) {
//             (Some(x0), Some(y0), Some(x1), Some(y1)) => {
//                 Some([Coordinate { x: x0, y: y0 }, Coordinate { x: x1, y: y1 }])
//             }
//             _ => None,
//         }
//     }

//     /// clears the bounding box.
//     fn clip_extent_clear(mut self) -> Self::OutputClear {
//         let base = self.base();

//         let base_out = ProjectionBuilder {
//             projection_raw: base.projection_raw,
//             clip: base.clip,
//             phi: base.phi,
//             lambda: base.lambda,
//             alpha: base.alpha,
//             k: base.k,
//             sx: base.sx,
//             sy: base.sy,
//             x: base.x,
//             y: base.y,
//             delta_lambda: base.delta_lambda,
//             delta_phi: base.delta_phi,
//             delta_gamma: base.delta_gamma,
//             delta2: base.delta2,
//             theta: base.theta,
//             rotate: base.rotate,
//             project_transform: base.project_transform,
//             project_rotate_transform: base.project_rotate_transform,
//             resample: base.resample,
//             rotate_transform: base.rotate_transform,

//             // Mutate stage
//             x0: None,
//             y0: None,
//             x1: None,
//             y1: None,
//             postclip: Identity::default(),
//         };

//         let out = MercatorBuilder {
//             pr: self.pr,
//             base: base_out,
//             x0: None,
//             y0: None,
//             x1: None,
//             y1: None, //
//         };
//         out.reset()
//     }

//     /// Sets the bounding box.
//     fn clip_extent(mut self, extent: &[Coordinate<Self::T>; 2]) -> Self::OutputBounded {
//         self.x0 = Some(extent[0].x);
//         self.y0 = Some(extent[0].y);
//         self.x1 = Some(extent[1].x);
//         self.y1 = Some(extent[1].y);
//         self.reclip()
//     }
// }

// // impl<DRAIN, INTERPOLATE, LINE, PR, PV, RESAMPLER, T> Angle
// //     for MercatorBuilder<DRAIN, INTERPOLATE, LINE, PR, PV, RESAMPLER, T>
// // where
// //     DRAIN: Default + Stream<EP = DRAIN, T = T>,
// //     INTERPOLATE: Interpolate<T = T>,
// //     LB: Line, LC: Line, LU: Line,
// //     PR: ProjectionRawBase<T>,
// //     PV: PointVisible<T = T>,
// //     RC: Resampler, RU: Resampler,
// //     T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// // {
// //     type T = T;

// //     #[inline]
// //     fn get_angle(&self) -> T {
// //         self.base.get_angle()
// //     }

// //     /// Sets the rotation angles as measured in degrees.
// //     fn angle(mut self, angle: T) -> Self {
// //         self.base = self.base.angle(angle);
// //         self
// //     }
// // }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Rotate
//     for MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     INTERPOLATE: Interpolate<T = T>,
//     LB: Line, LC: Line, LU: Line,
//     PR: ProjectionRawBase<T>,
//     PV: PointVisible<T = T>,
//     RC: Resampler, RU: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     type T = T;

//     #[inline]
//     fn get_rotate(&self) -> [T; 3] {
//         self.base.get_rotate()
//     }

//     /// Sets the rotation angles as measured in degrees.
//     fn rotate(mut self, angles: &[T; 3]) -> Self {
//         self.base = self.base.rotate(angles);
//         self
//     }
// }

// // impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Reflect
// //     for MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// // where
// //     DRAIN: Default + Stream<EP = DRAIN, T = T>,
// //     INTERPOLATE: Interpolate<T = T>,
// //     LB: Line, LC: Line, LU: Line,
// //     PR: ProjectionRawBase<T>,
// //     PV: PointVisible<T = T>,
// //     RC: Resampler, RU: Resampler,
// //     T: 'static
// //         + AbsDiffEq<Epsilon = T>
// //         + std::ops::AddAssign
// //         + AsPrimitive<T>
// //         + CoordFloat
// //         + Display
// //         + FloatConst,
// // {
// //     type T = T;

// //     /// Is the projection builder set to invert the x-coordinate.
// //     #[inline]
// //     fn get_reflect_x(&self) -> bool {
// //         self.base.get_reflect_x()
// //     }

// //     /// Set the projection builder to invert the x-coordinate.
// //     fn reflect_x(mut self, reflect: bool) -> Self {
// //         self.base = self.base.reflect_x(reflect);
// //         self
// //     }

// //     /// Is the projection builder set to invert the y-coordinate.
// //     #[inline]
// //     fn get_reflect_y(&self) -> bool {
// //         self.base.get_reflect_y()
// //     }

// //     /// Set the projection builder to invert the y-coordinate.
// //     #[inline]
// //     fn reflect_y(mut self, reflect: bool) -> Self {
// //         self.base = self.base.reflect_y(reflect);
// //         self
// //     }
// // }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> ClipAngle
//     for MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     INTERPOLATE: Interpolate<T = T>,
//     LB: Line, LC: Line, LU: Line,
//     PR: ProjectionRawBase<T>,
//     PV: PointVisible<T = T>,
//     RC: Resampler, RU: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type Output = MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>;
//     // type Drain = DRAIN;
//     // type Resampler = RESAMPLER;
//     // type PR = PR;
//     type T = T;

//     // Given an angle in degrees. Sets the internal clip angle and returns a builder
//     // which uses the clip circle stratergy.
//     fn clip_angle(self, angle: T) -> Self::Output {
//         self.base.clip_angle(angle);
//         self
//     }
// }
