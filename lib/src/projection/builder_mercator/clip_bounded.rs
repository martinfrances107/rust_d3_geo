// use std::marker::PhantomData;

// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use crate::clip::PointVisible;
// use crate::identity::Identity;
// use crate::projection::builder::template::ClipU;
// use crate::projection::builder::Builder as ProjectionBuilder;
// use crate::projection::builder_mercator::NoClipU;
// use crate::projection::ClipExtentBounded;
// use crate::projection::TransformExtent;

// use super::Builder;

// impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentBounded
//     for Builder<DRAIN, I, LB, LC, LU, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
// where
//     PR: TransformExtent<T = T>,
//     PV: PointVisible<T = T>,
//     T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     type OutputClear = Builder<DRAIN, I, LB, LC, LU, ClipU<DRAIN, T>, PR, PV, RC, RU, T>;
//     /// f64 or f32.
//     type T = T;

//     /// Returns a bounding box.

//     /// Clears the bounding box.
//     fn clip_extent_clear(self) -> Self::OutputClear {
//         let base = self.base;

//         let base_out = ProjectionBuilder {
//             p_lb: PhantomData::<LB>,
//             p_drain: PhantomData::<DRAIN>,
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
//             rotator: base.rotator,

//             // Mutate stage
//             x0: None,
//             y0: None,
//             x1: None,
//             y1: None,
//             postclip: Identity::default(),
//         };

//         let out = Builder {
//             pr: self.pr,
//             base: base_out,
//             extent: None,
//         };
//         // out.reset()
//         out
//     }
// }
