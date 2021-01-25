// use super::PathStream;
use crate::stream::Stream;
use geo::CoordFloat;
use num_traits::FloatConst;

use super::PathResult;
use super::PathResultEnum;

pub struct PathAreaStream<T>
where
    T: CoordFloat,
{
    area_sum: T,
    area_ring_sum: T,
    x00: T,
    y00: T,
    x0: T,
    y0: T,
    point_fn: fn(&mut Self, T, T),
    line_start_fn: fn(&mut Self),
    line_end_fn: fn(&mut Self),
}

impl<T> Default for PathAreaStream<T>
where
    T: CoordFloat + std::ops::AddAssign,
{
    fn default() -> Self {
        Self {
            area_sum: T::zero(),
            area_ring_sum: T::zero(),
            x00: T::zero(),
            y00: T::zero(),
            x0: T::zero(),
            y0: T::zero(),
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
    fn area_ring_start(&mut self) {
        self.point_fn = Self::area_point_first;
    }

    fn area_point_first(&mut self, x: T, y: T) {
        self.point_fn = Self::area_point;
        self.x00 = x;
        self.x0 = x;
        self.y00 = y;
        self.y0 = y;
    }

    fn area_point(&mut self, x: T, y: T) {
        self.area_ring_sum += self.y0 * x - self.x0 * y;
        self.x0 = x;
        self.y0 = y;
    }
    fn area_ring_end(&mut self) {}

    fn point_noop(&mut self, x_: T, y_: T) {}

    fn line_noop(&mut self) {}
}

impl<T> PathResult<T> for PathAreaStream<T>
where
    T: CoordFloat + std::ops::AddAssign,
{
    fn result(&self) -> PathResultEnum<T> {
        let area = self.area_sum / T::from(2).unwrap();
        self.area_sum = T::zero();
        return PathResultEnum::Area(area);
    }
}

impl<T> Stream<T> for PathAreaStream<T>
where
    T: CoordFloat + FloatConst + std::ops::AddAssign,
{
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
