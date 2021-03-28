mod intersect;
pub mod line;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::circle::circle_stream::circle_stream;
use crate::stream::CompareIntersection;
use crate::stream::Stream;

use super::clip::Clip;
use super::clip_base::ClipBase;
use super::ClipRaw;
use super::ClipTraitRaw;

// use line::Line;

#[derive(Clone, Debug)]
pub struct ClipCircle<T>
where
    T: CoordFloat + Default + FloatConst,
{
    radius: T,
    small_radius: bool,
    delta: T,
    cr: T,
    pub base: ClipBase<T>,
}

/// Returns a clip object
use std::fmt::Debug;
impl<T> ClipCircle<T>
where
    T: CoordFloat + Default + FloatConst + Debug,
{
    pub fn gen_clip(radius: T) -> Clip<T> {
        let cr = radius.cos();
        let small_radius = cr > T::zero();
        let start;
        if small_radius {
            start = Coordinate {
                x: T::zero(),
                y: T::zero() - radius,
            };
        } else {
            start = Coordinate {
                x: -T::PI(),
                y: radius - T::PI(),
            }
        }

        let cr = ClipRaw::Circle(ClipCircle {
            radius,
            delta: T::from(6u8).unwrap() * radius,
            small_radius,
            cr,
            base: ClipBase::default(),
        });

        Clip::new(cr, start)
    }
}

// impl<T> Stream<T> for ClipCircle<T>
// where
//     T: CoordFloat + Default + FloatConst,
// {
//     type C = Coordinate<T>;
// }

// impl<T> StreamPreClipTrait for ClipCircle<T>
// where
//     T: CoordFloat + Default + FloatConst + 'static,
// {
//     type SpctResample = ResampleEnum<T>;
//     // type SPCTstream = StreamSimpleNode<T>;
//     fn stream_resample_in(&mut self, _resample: Self::SpctResample) {
//         todo!("must connect");
//     }

//     fn box_clone(
//         &self,
//     ) -> Box<
//         dyn StreamPreClipTrait<
//             SctC = Self::SctC,
//             SctOC = Self::SctOC,
//             SctT = Self::SctT,
//             SctCi = Self::SctCi,
//             SctStream = Self::SctStream,
//             SpctResample = Self::SpctResample,
//         >,
//     > {
//         todo!("must clone");
//     }
// }
impl<T> ClipTraitRaw<T> for ClipCircle<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type SctC = Coordinate<T>;
    type SctOC = Option<Coordinate<T>>;
    // type SctStream = StreamSimpleNode<T>;
    // type SctStream = Stream<C = Coordinate<T>>;
    type SctT = T;
    type SctCi = CompareIntersection<T>;

    #[inline]
    fn point_visible(&self, p: &Coordinate<T>, _m: Option<u8>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }

    fn interpolate(
        &self,
        from: Self::SctOC,
        to: Self::SctOC,
        direction: Self::SctT,
        mut stream: impl Stream<T, C = Coordinate<T>>,
    ) {
        circle_stream(&mut stream, self.radius, self.delta, direction, from, to);
    }
}

// fn gen_clip_circle<T>(radius: T) -> Clip<T>
// where
//     T: CoordFloat + Default + FloatConst,
// {
//     let cr = radius.cos();
//     let smallRadius = cr > T::zero();
//     let start;
//     if smallRadius {
//         start = Coordinate {
//             x: T::zero,
//             y: std::ops::Neg(radius),
//         };
//     } else {
//         start = Coordinate {
//             x: -T::PI(),
//             y: radius - T::PI(),
//         }
//     }

//     let cr = ClipRaw::Circle(ClipCircle::new(radius));

//     Clip::new(cr, start)
// }
