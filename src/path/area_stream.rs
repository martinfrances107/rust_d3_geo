use derivative::Derivative;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::Result;
use super::ResultEnum;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
/// AreaStream is a pipeline terminating object ( a drain object ).
pub struct AreaStream<T>
where
    T: CoordFloat,
{
    area_sum: T,
    area_ring_sum: T,
    p00: Coordinate<T>,
    p0: Coordinate<T>,
    #[derivative(Debug = "ignore")]
    point_fn: fn(&mut Self, &Coordinate<T>),
    #[derivative(Debug = "ignore")]
    line_start_fn: fn(&mut Self),
    #[derivative(Debug = "ignore")]
    line_end_fn: fn(&mut Self),
}

impl<T> Default for AreaStream<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            area_sum: T::zero(),
            area_ring_sum: T::zero(),
            p0: Coordinate {
                x: T::nan(),
                y: T::nan(),
            },
            p00: Coordinate {
                x: T::nan(),
                y: T::nan(),
            },
            point_fn: Self::point_noop,
            line_start_fn: Self::line_noop,
            line_end_fn: Self::line_noop,
        }
    }
}

impl<T> AreaStream<T>
where
    T: CoordFloat,
{
    #[inline]
    fn area_ring_start(&mut self) {
        self.point_fn = Self::area_point_first;
    }

    fn area_point_first(&mut self, p: &Coordinate<T>) {
        self.point_fn = Self::area_point;
        self.p0 = *p;
        self.p00 = *p;
    }

    fn area_point(&mut self, p: &Coordinate<T>) {
        self.area_ring_sum = self.area_ring_sum + self.p0.y * p.x - self.p0.x * p.y;
        self.p0 = *p;
    }

    #[inline]
    fn area_ring_end(&mut self) {
        let p00 = self.p00;
        self.area_point(&p00);
    }

    #[inline]
    fn point_noop(&mut self, _p: &Coordinate<T>) {}

    #[inline]
    fn line_noop(&mut self) {}
}

impl<T> Result for AreaStream<T>
where
    T: CoordFloat,
{
    type Out = Option<ResultEnum<T>>;
    fn result(&mut self) -> Option<ResultEnum<T>> {
        let area = self.area_sum / T::from(2).unwrap();
        self.area_sum = T::zero();
        Some(ResultEnum::Area(area))
    }
}

impl<T> Stream for AreaStream<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        (self.point_fn)(self, p);
    }

    #[inline]
    fn line_start(&mut self) {
        (self.line_start_fn)(self);
    }

    #[inline]
    fn line_end(&mut self) {
        (self.line_end_fn)(self);
    }

    fn polygon_start(&mut self) {
        self.line_start_fn = Self::area_ring_start;
        self.line_end_fn = Self::area_ring_end;
    }
    fn polygon_end(&mut self) {
        self.line_start_fn = Self::line_noop;
        self.line_end_fn = Self::line_noop;
        self.point_fn = Self::point_noop;
        self.area_sum = self.area_sum + self.area_ring_sum.abs();
        self.area_ring_sum = T::zero();
    }
}
