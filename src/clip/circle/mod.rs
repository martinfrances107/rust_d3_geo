pub mod line;

mod intersect;

use std::fmt::Debug;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::circle::circle_stream::circle_stream;
use crate::stream::CompareIntersection;
use crate::stream::Stream;

use super::buffer::LineElem;
use super::clip::Clip;
use super::clip_base::ClipBase;
use super::clip_raw::ClipRaw;
use super::ClipTraitRaw;

#[derive(Clone, Debug)]
pub struct ClipCircle<T>
where
    T: AddAssign + CoordFloat + FloatConst + Default,
{
    radius: T,
    small_radius: bool,
    delta: T,
    cr: T,
    pub base: ClipBase<T>,
}

/// Returns a clip object
impl<T> ClipCircle<T>
where
    T: AddAssign + CoordFloat + FloatConst + Default + Debug,
{
    pub fn gen_clip(radius: T) -> Clip<T> {
        let cr = radius.cos();
        let small_radius = cr > T::zero();
        let start;
        if small_radius {
            start = LineElem {
                p: Coordinate {
                    x: T::zero(),
                    y: T::zero() - radius,
                },
                m: None,
            }
        } else {
            start = LineElem {
                p: Coordinate {
                    x: -T::PI(),
                    y: radius - T::PI(),
                },
                m: None,
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

impl<T> ClipTraitRaw<T> for ClipCircle<T>
where
    T: AddAssign + CoordFloat + FloatConst + Default,
{
    type SctC = Coordinate<T>;
    type SctOC = Option<Coordinate<T>>;
    type SctT = T;
    type SctCi = CompareIntersection<T>;

    #[inline]
    fn point_visible(&self, p: &Coordinate<T>, _m: Option<u8>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }

    #[inline]
    fn interpolate(
        &self,
        from: Self::SctOC,
        to: Self::SctOC,
        direction: Self::SctT,
        stream: &mut impl Stream<T, C = Coordinate<T>>,
    ) {
        circle_stream(stream, self.radius, self.delta, direction, from, to);
    }
}
