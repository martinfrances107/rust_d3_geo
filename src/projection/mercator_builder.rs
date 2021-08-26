// use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
// use crate::projection::resample::ResampleNode;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::FloatConst;

// use crate::clip::circle::line::Line as LineCircle;
// use crate::clip::circle::pv::PV;
// use crate::projection::clip_extent::ClipExtent;
// use crate::projection::mercator::Mercator;
// use crate::projection::scale::Scale;
// use crate::projection::Line;
// use crate::projection::PointVisible;
// use crate::projection::Raw as ProjectionRaw;
// use crate::stream::Stream;

// use crate::clip::circle::interpolate::generate as gen_interpolate;

// use super::builder::Builder as BuilderDefault;

// use std::any::type_name;

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

// pub fn gen_mercator_builder<DRAIN, PR, T>(
// ) -> MercatorBuilderGenerator<DRAIN, LineCircle<T>, PR, PV<T>, T>
// where
//     DRAIN: Stream<T = T>,
//     PR: ProjectionRaw<T>,
//     T: 'static + CoordFloat + FloatConst,
// {
//     let tau = T::from(2_f64).unwrap() * T::PI();
//     MercatorBuilderGenerator::new(
//         StreamNodeClipFactory::new(
//             gen_interpolate(T::one()),
//             LineCircle::<T>::default(),
//             PV::default(),
//         ),
//         PR::default(),
//     )
//     .scale(T::from(961_f64).unwrap() / tau)
// }

// /// Generates a projection builder.
// ///
// /// It takes a inner projector...
// ///
// #[derive(Debug)]
// struct MercatorBuilderGenerator<DRAIN, L, PR, PV, T>
// where
//     DRAIN: Stream<T = T>,
//     L: Line,
//     PV: PointVisible<T = T>,
//     PR: ProjectionRaw<T>,
//     T: 'static + CoordFloat + FloatConst,
// {
//     projection_raw: PR,
//     m: BuilderDefault<DRAIN, L, PR, PV, T>,

//     /// Clip Extent.
//     x0: Option<T>,
//     y0: T,
//     x1: T,
//     y1: T,
// }

// impl<DRAIN, L, PR, PV, T> MercatorBuilderGenerator<DRAIN, L, PR, PV, T>
// where
//     DRAIN: Stream<T = T>,
//     L: Line,
//     PR: ProjectionRaw<T>,
//     PV: PointVisible<T = T>,
//     T: 'static + CoordFloat + FloatConst,
// {
//     fn new(
//         preclip_factory: StreamNodeClipFactory<L, PR, PV, ResampleNode<PR, DRAIN, T>, T>,
//         projection_raw: PR,
//     ) -> Self {
//         Self {
//             projection_raw,
//             m: BuilderDefault::new(preclip_factory, projection_raw),
//             x0: None,
//             y0: T::nan(),
//             x1: T::nan(),
//             y1: T::nan(),
//         }
//     }

//     // fn is_of_type<X: 'static>(x: &dyn ProjectionRaw<T>) -> bool {
//     //     x.as_any().is::<X>()
//     // }

//     fn reclip(self) {
//         let k = T::PI() * self.m.get_scale();
//         // let t = rotate_radians(m.get_rotate()).invert(Coordinate {
//         //     x: T::zero(),
//         //     y: T::zero(),
//         // });

//         let t = Coordinate {
//             x: T::one(),
//             y: T::one(),
//         };

//         let ce = match self.x0 {
//             None => [
//                 Coordinate {
//                     x: t.x - k,
//                     y: t.y - k,
//                 },
//                 Coordinate {
//                     x: t.x + k,
//                     y: t.y + k,
//                 },
//             ],
//             Some(x0) => {
//                 let pr_type = type_of(self.projection_raw);
//                 dbg!("raw_type");
//                 match type_of(self.projection_raw) {
//                     "MERCATOR" => [
//                         Coordinate {
//                             x: (t.x - k).max(t.y - k),
//                             y: self.y0,
//                         },
//                         Coordinate {
//                             x: (t.x + k).min(self.x1),
//                             y: self.y1,
//                         },
//                     ],
//                     _ => [
//                         Coordinate {
//                             x: x0,
//                             y: (t.y - k).max(self.y0),
//                         },
//                         Coordinate {
//                             x: self.x1,
//                             y: (t.y + k).min(self.y1),
//                         },
//                     ],
//                 }
//             }
//         };

//         // self.m.clip_extent(Some(ce))
//     }
// }

// impl<DRAIN, L, PR, PV, T> Scale for MercatorBuilderGenerator<DRAIN, L, PR, PV, T>
// where
//     DRAIN: Stream<T = T>,
//     L: Line,
//     PR: ProjectionRaw<T>,
//     PV: PointVisible<T = T>,
//     T: 'static + CoordFloat + FloatConst,
// {
//     type T = T;
//     type Builder = Self;

//     #[inline]
//     fn get_scale(&self) -> T {
//         self.m.get_scale()
//     }

//     fn scale(mut self, scale: T) -> Self {
//         self.m.scale(scale);
//         self.reclip()
//     }
// }
