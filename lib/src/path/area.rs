use derivative::Derivative;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::Result;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
/// Stream Endpoint: Compute the area of the objects fed into the pipeline.
pub struct Area<T>
where
    T: CoordFloat,
{
    area_sum: T,
    area_ring_sum: T,
    p00: Coord<T>,
    p0: Coord<T>,
    #[derivative(Debug = "ignore")]
    point_fn: fn(&mut Self, &Coord<T>),
    #[derivative(Debug = "ignore")]
    line_start_fn: fn(&mut Self),
    #[derivative(Debug = "ignore")]
    line_end_fn: fn(&mut Self),
}

// Ignore the state machine functions.
impl<T> PartialEq for Area<T>
where
    T: CoordFloat,
{
    fn eq(&self, other: &Self) -> bool {
        self.area_sum == other.area_sum
            && self.area_ring_sum == other.area_ring_sum
            && self.p00 == other.p00
            && self.p0 == other.p0
    }
}

impl<T> Default for Area<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            area_sum: T::zero(),
            area_ring_sum: T::zero(),
            p0: Coord {
                x: T::nan(),
                y: T::nan(),
            },
            p00: Coord {
                x: T::nan(),
                y: T::nan(),
            },
            point_fn: Self::point_noop,
            line_start_fn: Self::line_noop,
            line_end_fn: Self::line_noop,
        }
    }
}

impl<T> Area<T>
where
    T: CoordFloat,
{
    #[inline]
    fn area_ring_start(&mut self) {
        self.point_fn = Self::area_point_first;
    }

    fn area_point_first(&mut self, p: &Coord<T>) {
        self.point_fn = Self::area_point;
        self.p0 = *p;
        self.p00 = *p;
    }

    fn area_point(&mut self, p: &Coord<T>) {
        self.area_ring_sum = self.area_ring_sum + self.p0.y * p.x - self.p0.x * p.y;
        self.p0 = *p;
    }

    #[inline]
    fn area_ring_end(&mut self) {
        let p00 = self.p00;
        self.area_point(&p00);
    }

    #[inline]
    #[allow(clippy::unused_self)]
    fn point_noop(&mut self, _p: &Coord<T>) {}

    #[inline]
    #[allow(clippy::unused_self)]
    fn line_noop(&mut self) {}
}

impl<T> Result for Area<T>
where
    T: CoordFloat,
{
    type Out = T;

    fn result(&mut self) -> Self::Out {
        let area = self.area_sum / T::from(2).unwrap();
        self.area_sum = T::zero();
        area
    }
}

impl<T> Stream for Area<T>
where
    T: CoordFloat,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
        self
    }

    #[inline]
    fn line_end(&mut self) {
        (self.line_end_fn)(self);
    }

    #[inline]
    fn line_start(&mut self) {
        (self.line_start_fn)(self);
    }

    #[inline]
    fn point(&mut self, p: &Coord<T>, _m: Option<u8>) {
        (self.point_fn)(self, p);
    }

    fn polygon_end(&mut self) {
        self.line_start_fn = Self::line_noop;
        self.line_end_fn = Self::line_noop;
        self.point_fn = Self::point_noop;
        self.area_sum = self.area_sum + self.area_ring_sum.abs();
        self.area_ring_sum = T::zero();
    }
    fn polygon_start(&mut self) {
        self.line_start_fn = Self::area_ring_start;
        self.line_end_fn = Self::area_ring_end;
    }
}
