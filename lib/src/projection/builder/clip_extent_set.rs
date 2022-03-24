// use approx::AbsDiffEq;

// use geo::CoordFloat;
// use geo::Coordinate;

// use num_traits::FloatConst;

// use crate::clip::rectangle::Rectangle;
// use crate::clip::Interpolate;
// use crate::clip::PointVisible;
// use crate::projection::resampler::Resampler;
// use crate::stream::Connectable;
// use crate::stream::Stream;
// use crate::stream::Unconnected;

// use super::Builder;
// use super::ClipExtentSet;

// use super::template::ClipC;
// use super::template::ClipU;
// use super::template::NoClipC;
// use super::template::NoClipU;
// use super::ProjectionRawBase;

// impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentSet
//     for Builder<DRAIN, I, LB, LC, LU, NoClipC<DRAIN, T>, NoClipU<DRAIN, T>, PR, PV, RC, RU, T>
// where
//     I: Interpolate<T = T>,
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     PR: ProjectionRawBase<T>,
//     PV: PointVisible<T = T>,
//     RC: Resampler + Stream<EP = DRAIN, T = T>,
//     RU: Resampler + Connectable<Output = RC, SC = NoClipC<DRAIN, T>>,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type T = T;

//     type OutputBounded =
//         Builder<DRAIN, I, LB, LC, LU, ClipC<DRAIN, T>, ClipU<DRAIN, T>, PR, PV, RC, RU, T>;

//     fn clip_extent(mut self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
//         let out = Self::OutputBounded {
//             // p_pcnc: self.p_pcnc,
//             projection_raw: self.projection_raw,
//             clip: self.clip,
//             phi: self.phi,
//             lambda: self.lambda,
//             alpha: self.alpha,
//             k: self.k,
//             sx: self.sx,
//             sy: self.sy,
//             x: self.x,
//             y: self.y,
//             delta_lambda: self.delta_lambda,
//             delta_phi: self.delta_phi,
//             delta_gamma: self.delta_gamma,
//             delta2: self.delta2,
//             theta: self.theta,
//             rotate: self.rotate,
//             project_transform: self.project_transform,
//             project_rotate_transform: self.project_rotate_transform,
//             resample: self.resample,
//             rotate_transform: self.rotate_transform,

//             // Mutate stage
//             x0: Some(extent[0].x),
//             y0: Some(extent[0].y),
//             x1: Some(extent[1].x),
//             y1: Some(extent[1].y),
//             postclip: Rectangle::<DRAIN, DRAIN, Unconnected, T>::new(
//                 extent[0].x,
//                 extent[0].y,
//                 extent[1].x,
//                 extent[1].y,
//             ),
//         };

//         out.reset()
//     }
// }
