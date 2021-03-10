mod intersect;
pub mod line;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::circle::circle_stream::circle_stream;
use crate::projection::resample::ResampleEnum;
use crate::stream::CompareIntersection;
use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::stream::StreamPreClipTrait;
use crate::stream::StreamSimpleNode;

use super::clip_base::ClipBase;
// use super::BufferInTrait;
// use super::ClipBuffer;
use super::ClipTraitRaw;
// use super::LineEnum;

use super::clip::Clip;
use super::ClipRaw;

// use line::Line;

#[derive(Clone)]
pub struct ClipCircle<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
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
    T: CoordFloat + FloatConst + Default + Debug + 'static,
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
impl<T> StreamClone for ClipCircle<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(self.clone())
    }
}

impl<T> Stream for ClipCircle<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type C = Coordinate<T>;
}

impl<T> StreamPreClipTrait for ClipCircle<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type SpctResample = ResampleEnum<T>;
    // type SPCTstream = StreamSimpleNode<T>;
    fn stream_resample_in(&mut self, _resample: Self::SpctResample) {
        todo!("must connect");
    }

    fn box_clone(
        &self,
    ) -> Box<
        dyn StreamPreClipTrait<
            SctC = Self::SctC,
            SctOC = Self::SctOC,
            SctT = Self::SctT,
            SctCi = Self::SctCi,
            SctStream = Self::SctStream,
            SpctResample = Self::SpctResample,
        >,
    > {
        todo!("must clone");
    }
}
impl<T> ClipTraitRaw for ClipCircle<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type SctC = Coordinate<T>;
    type SctOC = Option<Coordinate<T>>;
    type SctStream = StreamSimpleNode<T>;
    type SctT = T;
    type SctCi = CompareIntersection<T>;

    #[inline]
    fn point_visible(&self, p: Coordinate<T>, _m: Option<u8>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }

    fn interpolate(
        &self,
        from: Self::SctOC,
        to: Self::SctOC,
        direction: Self::SctT,
        mut stream: Self::SctStream,
    ) {
        circle_stream(&mut stream, self.radius, self.delta, direction, from, to);
    }
}

// fn gen_clip_circle<T>(radius: T) -> Clip<T>
// where
//     T: CoordFloat + FloatConst + Default,
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
