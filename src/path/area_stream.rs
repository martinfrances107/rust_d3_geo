use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Stream;
use crate::stream::StreamDst;

use super::PathResult;
use super::PathResultEnum;

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct PathAreaStream<T>
where
    T: CoordFloat,
{
    area_sum: T,
    area_ring_sum: T,
    p00: Coordinate<T>,
    p0: Coordinate<T>,
    #[derivative(Debug = "ignore")]
    point_fn: fn(&mut Self, &Coordinate<T>, Option<u8>),
    #[derivative(Debug = "ignore")]
    line_start_fn: fn(&mut Self),
    #[derivative(Debug = "ignore")]
    line_end_fn: fn(&mut Self),
}

impl<T> Default for PathAreaStream<T>
where
    T: CoordFloat + Default + std::ops::AddAssign,
{
    #[inline]
    fn default() -> Self {
        Self {
            area_sum: T::zero(),
            area_ring_sum: T::zero(),
            p0: Coordinate::default(),
            p00: Coordinate::default(),
            point_fn: Self::point_noop,
            line_start_fn: Self::line_noop,
            line_end_fn: Self::line_noop,
        }
    }
}

impl<T> PathAreaStream<T>
where
    T: CoordFloat + std::ops::AddAssign,
{
    #[inline]
    fn area_ring_start(&mut self) {
        self.point_fn = Self::area_point_first;
    }

    fn area_point_first(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        self.point_fn = Self::area_point;
        self.p0 = *p;
        self.p00 = *p;
    }

    fn area_point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        self.area_ring_sum += self.p0.y * p.x - self.p0.x * p.y;
        self.p0 = *p;
    }

    #[inline]
    fn area_ring_end(&mut self) {
        let p00 = self.p00.clone();
        self.area_point(&p00, None);
    }

    #[inline]
    fn point_noop(&mut self, _p: &Coordinate<T>, _m: Option<u8>) {}

    #[inline]
    fn line_noop(&mut self) {}
}

impl<T> PathResult for PathAreaStream<T>
where
    T: CoordFloat + std::ops::AddAssign,
{
    type Out = Option<PathResultEnum<T>>;
    fn result(&mut self) -> Option<PathResultEnum<T>> {
        let area = self.area_sum / T::from(2).unwrap();
        self.area_sum = T::zero();
        Some(PathResultEnum::Area(area))
    }
}

impl<T> Stream<T> for PathAreaStream<T>
where
    T: CoordFloat + Default + FloatConst + std::ops::AddAssign,
{
    type C = Coordinate<T>;

    fn sphere(&mut self) {}

    fn get_dst(&self) -> StreamDst<T> {
        StreamDst::PAS(self.clone())
    }

    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        (self.point_fn)(self, p, m);
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
        self.area_sum += self.area_ring_sum.abs();
        self.area_ring_sum = T::zero();
    }
}
