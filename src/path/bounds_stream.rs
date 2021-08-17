use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::Result;
use super::ResultEnum;

// #[derive(Derivative)]
// #[derivative(Debug)]
#[derive(Clone, Debug)]
pub struct BoundsStream<T>
where
    T: CoordFloat,
{
    p0: Coordinate<T>,
    p1: Coordinate<T>,
}

impl<T> Default for BoundsStream<T>
where
    T: AddAssign + CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            p0: Coordinate {
                x: T::infinity(),
                y: T::infinity(),
            },
            p1: Coordinate {
                x: -T::infinity(),
                y: -T::infinity(),
            },
        }
    }
}

impl<T> Result for BoundsStream<T>
where
    T: AddAssign + CoordFloat,
{
    type Out = Option<ResultEnum<T>>;
    fn result(&mut self) -> Option<ResultEnum<T>> {
        let bounds = [self.p0, self.p1];
        Some(ResultEnum::Bounds(bounds));
        *self = Self::default();
        Some(ResultEnum::Bounds(bounds))
    }
}

impl<T> Stream for BoundsStream<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        if p.x < self.p0.x {
            self.p0.x = p.x
        }
        if p.x > self.p1.x {
            self.p1.x = p.x
        }
        if p.y < self.p0.y {
            self.p0.y = p.y
        }
        if p.y > self.p1.y {
            self.p1.y = p.y
        }
    }

    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
    // fn get_dst(&self) -> BoundsStream<T> {
    //     self.clone()
    // }
}
