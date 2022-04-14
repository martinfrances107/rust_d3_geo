// use std::fmt::Debug;

// use approx::AbsDiffEq;
// use core::marker::PhantomData;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::FloatConst;

// use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
// use crate::clip::antimeridian::line::Line as LineAntimeridian;
// use crate::clip::antimeridian::pv::PV as PVAntimeridian;
// use crate::clip::rectangle::Rectangle;
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
// use crate::projection::resampler::resample::Resample;
// use crate::projection::TranslateSet;
// use crate::stream::Connected;
// use crate::stream::Unconnected;
// use crate::Transform;

// use super::template::ClipC;
// use super::template::ClipU;
// use super::template::NoClipC;
// use super::template::NoClipU;
// use super::Builder;

// impl<DRAIN, PR, PV, T> TranslateSet
//     for Builder<
//         DRAIN,
//         InterpolateAntimeridian<T>,
//         LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//         LineAntimeridian<
//             DRAIN,
//             ResampleNoClipC<DRAIN, PR, T>,
//             Connected<ResampleNoClipC<DRAIN, PR, T>>,
//             T,
//         >,
//         LineAntimeridian<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
//         NoClipC<DRAIN, T>,
//         NoClipU<DRAIN, T>,
//         PR,
//         PV,
//         ResampleNoClipC<DRAIN, PR, T>,
//         ResampleNoClipU<DRAIN, PR, T>,
//         T,
//     >
// where
//     DRAIN: Debug,
//     PR: Clone + Debug + Transform<T = T>,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type Output = Builder<
//         DRAIN,
//         InterpolateAntimeridian<T>,
//         LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//         LineAntimeridian<
//             DRAIN,
//             ResampleNoClipC<DRAIN, PR, T>,
//             Connected<ResampleNoClipC<DRAIN, PR, T>>,
//             T,
//         >,
//         LineAntimeridian<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
//         ClipC<DRAIN, T>,
//         ClipU<DRAIN, T>,
//         PR,
//         PVAntimeridian<T>,
//         ResampleClipC<DRAIN, PR, T>,
//         ResampleClipU<DRAIN, PR, T>,
//         T,
//     >;
//     type T = T;

//     fn translate(mut self, t: &Coordinate<T>) -> Self::Output {
//         // self.x = t.x;
//         // self.y = t.y;
//         // self.recenter_with_resampling()

//         // types have changes rebuild clip, resample.
//         let pv = PVAntimeridian::default();
//         let interpolator = InterpolateAntimeridian::default();
//         let line = LineAntimeridian::default();

//         let resample = Resample::new(self.project_transform.clone(), self.delta2);
//         // Architecture Discussion:
//         // CLIP is generic over <.. RC, RU,..>,
//         // So a change in the resample type causes rebuilding of clip.
//         let clip = Clip::new(interpolator, line, pv, self.clip.start);

//         let out = Self::Output {
//             // Modification by input.
//             x: t.x,
//             y: t.y, // translate
//             postclip: Rectangle::new(
//                 self.x0.unwrap(),
//                 self.y0.unwrap(),
//                 self.x1.unwrap(),
//                 self.y1.unwrap(),
//             ),

//             p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
//             projection_raw: self.projection_raw,
//             clip,
//             lambda: self.lambda,
//             phi: self.phi,
//             alpha: self.alpha, // post-rotate angle
//             k: self.k,         // scale
//             sx: self.sx,       // reflectX
//             sy: self.sy,       // reflectY

//             delta_lambda: self.delta_lambda,
//             delta_phi: self.delta_phi,
//             delta_gamma: self.delta_gamma,
//             delta2: self.delta2, // precision
//             theta: self.theta,
//             x0: self.x0,
//             y0: self.y0,
//             x1: self.x1,
//             y1: self.y1, // post-clip extent

//             /// Used by recenter() to build the factories.
//             rotate: self.rotate,
//             rotator: self.rotator, //rotate, pre-rotate
//             project_transform: self.project_transform,
//             /// Used by rotate_transform_factory and projections transform.
//             project_rotate_transform: self.project_rotate_transform,

//             /// Projection pipeline stage
//             resample,
//         };

//         out.recenter_with_resampling()
//     }
// }

// impl<DRAIN, PR, PV, T> TranslateSet
//     for Builder<
//         DRAIN,
//         InterpolateAntimeridian<T>,
//         LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//         LineAntimeridian<
//             DRAIN,
//             ResampleNoneNoClipC<DRAIN, PR, T>,
//             Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
//             T,
//         >,
//         LineAntimeridian<DRAIN, ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
//         NoClipC<DRAIN, T>,
//         NoClipU<DRAIN, T>,
//         PR,
//         PV,
//         ResampleNoneNoClipC<DRAIN, PR, T>,
//         ResampleNoneNoClipU<DRAIN, PR, T>,
//         T,
//     >
// where
//     DRAIN: Debug,
//     PR: Clone + Debug + Transform<T = T>,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type Output = Builder<
//         DRAIN,
//         InterpolateAntimeridian<T>,
//         LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
//         LineAntimeridian<
//             DRAIN,
//             ResampleNoneNoClipC<DRAIN, PR, T>,
//             Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
//             T,
//         >,
//         LineAntimeridian<DRAIN, ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
//         ClipC<DRAIN, T>,
//         ClipU<DRAIN, T>,
//         PR,
//         PVAntimeridian<T>,
//         ResampleNoneClipC<DRAIN, PR, T>,
//         ResampleNoneClipU<DRAIN, PR, T>,
//         T,
//     >;
//     type T = T;

//     fn translate(mut self, t: &Coordinate<T>) -> Self::Output {
//         // self.x = t.x;
//         // self.y = t.y;
//         // self.recenter_with_resampling()

//         // types have changes rebuild clip, resample.
//         let pv = PVAntimeridian::default();
//         let interpolator = InterpolateAntimeridian::default();
//         let line = LineAntimeridian::default();

//         let resample = None::new(self.project_transform.clone());
//         // Architecture Discussion:
//         // CLIP is generic over <.. RC, RU,..>,
//         // So a change in the resample type causes rebuilding of clip.
//         let clip = Clip::new(interpolator, line, pv, self.clip.start);

//         let out = Self::Output {
//             // Modification by input.
//             x: t.x,
//             y: t.y, // translate
//             postclip: Rectangle::new(
//                 self.x0.unwrap(),
//                 self.y0.unwrap(),
//                 self.x1.unwrap(),
//                 self.y1.unwrap(),
//             ),

//             p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
//             projection_raw: self.projection_raw,
//             clip,
//             lambda: self.lambda,
//             phi: self.phi,
//             alpha: self.alpha, // post-rotate angle
//             k: self.k,         // scale
//             sx: self.sx,       // reflectX
//             sy: self.sy,       // reflectY

//             delta_lambda: self.delta_lambda,
//             delta_phi: self.delta_phi,
//             delta_gamma: self.delta_gamma,
//             delta2: self.delta2, // precision
//             theta: self.theta,
//             x0: self.x0,
//             y0: self.y0,
//             x1: self.x1,
//             y1: self.y1, // post-clip extent

//             /// Used by recenter() to build the factories.
//             rotate: self.rotate,
//             rotator: self.rotator, //rotate, pre-rotate
//             project_transform: self.project_transform,
//             /// Used by rotate_transform_factory and projections transform.
//             project_rotate_transform: self.project_rotate_transform,

//             /// Projection pipeline stage
//             resample,
//         };

//         out.recenter_no_resampling()
//     }
// }
