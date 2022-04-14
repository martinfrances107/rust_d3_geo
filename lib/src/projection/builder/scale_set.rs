// use std::fmt::Debug;
// use std::marker::PhantomData;

// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use num_traits::FloatConst;

// use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
// use crate::clip::antimeridian::line::Line as LineAntimeridian;
// use crate::clip::antimeridian::pv::PV as PVAntimeridian;
// use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
// use crate::clip::circle::line::Line as LineCircle;
// use crate::clip::circle::pv::PV as PVCircle;
// use crate::clip::rectangle::Rectangle;
// use crate::identity::Identity;
// use crate::projection::builder::template::ResampleClipC;
// use crate::projection::builder::template::ResampleClipU;
// use crate::projection::builder::template::ResampleNoneClipC;
// use crate::projection::builder::template::ResampleNoneClipU;
// use crate::projection::builder::template::ResampleNoneNoClipC;
// use crate::projection::builder::template::ResampleNoneNoClipU;
// use crate::projection::builder::Buffer;
// use crate::projection::builder::Clip;
// use crate::projection::builder::ResampleNoClipC;
// use crate::projection::builder::ResampleNoClipU;
// use crate::projection::resampler::none::None;
// use crate::projection::resampler::none::None as ResampleNone;
// use crate::projection::resampler::resample::Connected as ConnectedResample;
// use crate::projection::resampler::resample::Resample;
// use crate::projection::ScaleSet;
// use crate::stream::Connectable;
// use crate::stream::Connected;
// use crate::stream::Stream;
// use crate::stream::Unconnected;
// use crate::Transform;

// use super::template::ClipC;
// use super::template::ClipU;
// use super::template::NoClipC;
// use super::template::NoClipU;
// use super::Builder;

// // TODO must vary by ClipCircle.
// impl<DRAIN, PR, T> ScaleSet
//         for Builder<
//                 DRAIN,
//                 InterpolateAntimeridian<T>,
//                 LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//                 LineAntimeridian<
//                         DRAIN,
//                         ResampleNoClipC<DRAIN, PR, T>,
//                         Connected<ResampleNoClipC<DRAIN, PR, T>>,
//                         T,
//                 >,
//                 LineAntimeridian<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
//                 NoClipC<DRAIN, T>,
//                 NoClipU<DRAIN, T>,
//                 PR,
//                 PVAntimeridian<T>,
//                 ResampleNoClipC<DRAIN, PR, T>,
//                 ResampleNoClipU<DRAIN, PR, T>,
//                 T,
//         >
// where
//         DRAIN: Debug,
//         PR: Clone + Debug + Transform<T = T>,
//         T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//         type Output = Builder<
//                 DRAIN,
//                 InterpolateAntimeridian<T>,
//                 LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//                 LineAntimeridian<
//                         DRAIN,
//                         ResampleNoClipC<DRAIN, PR, T>,
//                         Connected<ResampleNoClipC<DRAIN, PR, T>>,
//                         T,
//                 >,
//                 LineAntimeridian<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
//                 NoClipC<DRAIN, T>,
//                 NoClipU<DRAIN, T>,
//                 PR,
//                 PVAntimeridian<T>,
//                 ResampleNoClipC<DRAIN, PR, T>,
//                 ResampleNoClipU<DRAIN, PR, T>,
//                 T,
//         >;
//         type T = T;

//         fn scale(mut self, scale: T) -> Self::Output {
//                 self.k = scale;

//                 // types have changes rebuild clip, resample.
//                 let pv = PVAntimeridian::default();
//                 let interpolator = InterpolateAntimeridian::default();
//                 let line = LineAntimeridian::default();

//                 let resample = Resample::new(self.project_transform.clone(), self.delta2);
//                 // Architecture Discussion:
//                 // CLIP is generic over <.. RC, RU,..>,
//                 // So a change in the resample type causes rebuilding of clip.
//                 let clip = Clip::new(interpolator, line, pv, self.clip.start);

//                 let out = Self::Output {
//                         // Modification by input.
//                         k: self.k, // scale
//                         clip,

//                         p_pcnc: PhantomData::<NoClipC<DRAIN, T>>,
//                         projection_raw: self.projection_raw,

//                         lambda: self.lambda,
//                         phi: self.phi,
//                         alpha: self.alpha, // post-rotate angle
//                         x: self.x,
//                         y: self.y,   // translate
//                         sx: self.sx, // reflectX
//                         sy: self.sy, // reflectY

//                         delta_lambda: self.delta_lambda,
//                         delta_phi: self.delta_phi,
//                         delta_gamma: self.delta_gamma,
//                         delta2: self.delta2, // precision
//                         theta: self.theta,
//                         x0: self.x0,
//                         y0: self.y0,
//                         x1: self.x1,
//                         y1: self.y1, // post-clip extent

//                         /// Used by recenter() to build the factories.
//                         rotate: self.rotate,
//                         rotator: self.rotator, //rotate, pre-rotate
//                         project_transform: self.project_transform,
//                         /// Used by rotate_transform_factory and projections transform.
//                         project_rotate_transform: self.project_rotate_transform,
//                         postclip: Identity::default(),

//                         /// Projection pipeline stage
//                         resample,
//                 };
//                 out.recenter_with_resampling()
//         }
// }

// impl<DRAIN, PR, T> ScaleSet
//         for Builder<
//                 DRAIN,
//                 InterpolateAntimeridian<T>,
//                 LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//                 LineCircle<
//                         DRAIN,
//                         ResampleNoClipC<DRAIN, PR, T>,
//                         Connected<ResampleNoClipC<DRAIN, PR, T>>,
//                         T,
//                 >,
//                 LineCircle<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
//                 NoClipC<DRAIN, T>,
//                 NoClipU<DRAIN, T>,
//                 PR,
//                 PVCircle<T>,
//                 ResampleNoClipC<DRAIN, PR, T>,
//                 ResampleNoClipU<DRAIN, PR, T>,
//                 T,
//         >
// where
//         DRAIN: Debug,
//         PR: Clone + Debug + Transform<T = T>,
//         T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//         type Output = Builder<
//                 DRAIN,
//                 InterpolateAntimeridian<T>,
//                 LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//                 LineAntimeridian<
//                         DRAIN,
//                         ResampleClipC<DRAIN, PR, T>,
//                         Connected<ResampleClipC<DRAIN, PR, T>>,
//                         T,
//                 >,
//                 LineAntimeridian<DRAIN, ResampleClipC<DRAIN, PR, T>, Unconnected, T>,
//                 ClipC<DRAIN, T>,
//                 ClipU<DRAIN, T>,
//                 PR,
//                 PVAntimeridian<T>,
//                 ResampleClipC<DRAIN, PR, T>,
//                 ResampleClipU<DRAIN, PR, T>,
//                 T,
//         >;
//         type T = T;

//         fn scale(mut self, scale: T) -> Self::Output {
//                 self.k = scale;

//                 // types have changes rebuild clip, resample.
//                 let pv = PVAntimeridian::default();
//                 let interpolator = InterpolateAntimeridian::default();
//                 let line = LineAntimeridian::default();

//                 let resample = Resample::new(self.project_transform.clone(), self.delta2);
//                 // Architecture Discussion:
//                 // CLIP is generic over <.. RC, RU,..>,
//                 // So a change in the resample type causes rebuilding of clip.
//                 let clip = Clip::new(interpolator, line, pv, self.clip.start);

//                 let out = Self::Output {
//                         // Modification by input.
//                         k: self.k, // scale
//                         clip,

//                         p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
//                         projection_raw: self.projection_raw,

//                         lambda: self.lambda,
//                         phi: self.phi,
//                         alpha: self.alpha, // post-rotate angle
//                         x: self.x,
//                         y: self.y,   // translate
//                         sx: self.sx, // reflectX
//                         sy: self.sy, // reflectY

//                         delta_lambda: self.delta_lambda,
//                         delta_phi: self.delta_phi,
//                         delta_gamma: self.delta_gamma,
//                         delta2: self.delta2, // precision
//                         theta: self.theta,
//                         x0: self.x0,
//                         y0: self.y0,
//                         x1: self.x1,
//                         y1: self.y1, // post-clip extent

//                         /// Used by recenter() to build the factories.
//                         rotate: self.rotate,
//                         rotator: self.rotator, //rotate, pre-rotate
//                         project_transform: self.project_transform,
//                         /// Used by rotate_transform_factory and projections transform.
//                         project_rotate_transform: self.project_rotate_transform,
//                         postclip: Rectangle::new(
//                                 self.x0.unwrap(),
//                                 self.y0.unwrap(),
//                                 self.x1.unwrap(),
//                                 self.y1.unwrap(),
//                         ),

//                         /// Projection pipeline stage
//                         resample,
//                 };
//                 out.recenter_with_resampling()
//         }
// }

// /// above resampling, below -- None
// impl<DRAIN, PR, T> ScaleSet
//         for Builder<
//                 DRAIN,
//                 InterpolateAntimeridian<T>,
//                 LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//                 LineAntimeridian<
//                         DRAIN,
//                         ResampleNoneNoClipC<DRAIN, PR, T>,
//                         Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
//                         T,
//                 >,
//                 LineAntimeridian<DRAIN, ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
//                 NoClipC<DRAIN, T>,
//                 NoClipU<DRAIN, T>,
//                 PR,
//                 PVAntimeridian<T>,
//                 ResampleNoneNoClipC<DRAIN, PR, T>,
//                 ResampleNoneNoClipU<DRAIN, PR, T>,
//                 T,
//         >
// where
//         DRAIN: Debug,
//         PR: Clone + Debug + Transform<T = T>,
//         T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//         type Output = Builder<
//                 DRAIN,
//                 InterpolateAntimeridian<T>,
//                 LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//                 LineAntimeridian<
//                         DRAIN,
//                         ResampleNoneNoClipC<DRAIN, PR, T>,
//                         Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
//                         T,
//                 >,
//                 LineAntimeridian<DRAIN, ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
//                 NoClipC<DRAIN, T>,
//                 NoClipU<DRAIN, T>,
//                 PR,
//                 PVAntimeridian<T>,
//                 ResampleNoneNoClipC<DRAIN, PR, T>,
//                 ResampleNoneNoClipU<DRAIN, PR, T>,
//                 T,
//         >;
//         type T = T;

//         fn scale(mut self, scale: T) -> Self::Output {
//                 self.k = scale;

//                 // types have changes rebuild clip, resample.
//                 let pv = PVAntimeridian::default();
//                 let interpolator = InterpolateAntimeridian::default();
//                 let line = LineAntimeridian::default();

//                 // let resample = Resample::new(self.project_transform.clone(), self.delta2);
//                 let resample = None::new(self.project_transform.clone());
//                 // Architecture Discussion:
//                 // CLIP is generic over <.. RC, RU,..>,
//                 // So a change in the resample type causes rebuilding of clip.
//                 let clip = Clip::new(interpolator, line, pv, self.clip.start);

//                 let out = Self::Output {
//                         // Modification by input.
//                         k: self.k, // scale
//                         clip,

//                         p_pcnc: PhantomData::<NoClipC<DRAIN, T>>,
//                         projection_raw: self.projection_raw,

//                         lambda: self.lambda,
//                         phi: self.phi,
//                         alpha: self.alpha, // post-rotate angle
//                         x: self.x,
//                         y: self.y,   // translate
//                         sx: self.sx, // reflectX
//                         sy: self.sy, // reflectY

//                         delta_lambda: self.delta_lambda,
//                         delta_phi: self.delta_phi,
//                         delta_gamma: self.delta_gamma,
//                         delta2: self.delta2, // precision
//                         theta: self.theta,
//                         x0: self.x0,
//                         y0: self.y0,
//                         x1: self.x1,
//                         y1: self.y1, // post-clip extent

//                         /// Used by recenter() to build the factories.
//                         rotate: self.rotate,
//                         rotator: self.rotator, //rotate, pre-rotate
//                         project_transform: self.project_transform,
//                         /// Used by rotate_transform_factory and projections transform.
//                         project_rotate_transform: self.project_rotate_transform,
//                         postclip: Identity::default(),

//                         /// Projection pipeline stage
//                         resample,
//                 };
//                 out.recenter_no_resampling()
//         }
// }

// impl<DRAIN, PR, T> ScaleSet
//         for Builder<
//                 DRAIN,
//                 InterpolateAntimeridian<T>,
//                 LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//                 LineAntimeridian<
//                         DRAIN,
//                         ResampleNoneClipC<DRAIN, PR, T>,
//                         Connected<ResampleNoneClipC<DRAIN, PR, T>>,
//                         T,
//                 >,
//                 LineAntimeridian<DRAIN, ResampleNoneClipC<DRAIN, PR, T>, Unconnected, T>,
//                 ClipC<DRAIN, T>,
//                 ClipU<DRAIN, T>,
//                 PR,
//                 PVAntimeridian<T>,
//                 ResampleNoneClipC<DRAIN, PR, T>,
//                 ResampleNoneClipU<DRAIN, PR, T>,
//                 T,
//         >
// where
//         DRAIN: Debug,
//         PR: Clone + Debug + Transform<T = T>,
//         T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//         type Output = Builder<
//                 DRAIN,
//                 InterpolateAntimeridian<T>,
//                 LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//                 LineAntimeridian<
//                         DRAIN,
//                         ResampleNoneClipC<DRAIN, PR, T>,
//                         Connected<ResampleNoneClipC<DRAIN, PR, T>>,
//                         T,
//                 >,
//                 LineAntimeridian<DRAIN, ResampleNoneClipC<DRAIN, PR, T>, Unconnected, T>,
//                 ClipC<DRAIN, T>,
//                 ClipU<DRAIN, T>,
//                 PR,
//                 PVAntimeridian<T>,
//                 ResampleNoneClipC<DRAIN, PR, T>,
//                 ResampleNoneClipU<DRAIN, PR, T>,
//                 T,
//         >;
//         type T = T;

//         fn scale(mut self, scale: T) -> Self::Output {
//                 self.k = scale;

//                 // types have changes rebuild clip, resample.
//                 let pv = PVAntimeridian::default();
//                 let interpolator = InterpolateAntimeridian::default();
//                 let line = LineAntimeridian::default();

//                 let resample = None::new(self.project_transform.clone());
//                 // Architecture Discussion:
//                 // CLIP is generic over <.. RC, RU,..>,
//                 // So a change in the resample type causes rebuilding of clip.
//                 let clip = Clip::new(interpolator, line, pv, self.clip.start);

//                 let out = Self::Output {
//                         // Modification by input.
//                         k: self.k, // scale
//                         clip,

//                         p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
//                         projection_raw: self.projection_raw,

//                         lambda: self.lambda,
//                         phi: self.phi,
//                         alpha: self.alpha, // post-rotate angle
//                         x: self.x,
//                         y: self.y,   // translate
//                         sx: self.sx, // reflectX
//                         sy: self.sy, // reflectY

//                         delta_lambda: self.delta_lambda,
//                         delta_phi: self.delta_phi,
//                         delta_gamma: self.delta_gamma,
//                         delta2: self.delta2, // precision
//                         theta: self.theta,
//                         x0: self.x0,
//                         y0: self.y0,
//                         x1: self.x1,
//                         y1: self.y1, // post-clip extent

//                         /// Used by recenter() to build the factories.
//                         rotate: self.rotate,
//                         rotator: self.rotator, //rotate, pre-rotate
//                         project_transform: self.project_transform,
//                         /// Used by rotate_transform_factory and projections transform.
//                         project_rotate_transform: self.project_rotate_transform,
//                         postclip: Rectangle::new(
//                                 self.x0.unwrap(),
//                                 self.y0.unwrap(),
//                                 self.x1.unwrap(),
//                                 self.y1.unwrap(),
//                         ),

//                         /// Projection pipeline stage
//                         resample,
//                 };
//                 out.recenter_no_resampling()
//         }
// }
