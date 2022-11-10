use std::ops::AddAssign;

use derivative::Derivative;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::Result;

#[derive(Clone, Debug)]
enum MeasureMode {
    None,
    Polygon,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
/// Stream Endpoint: Compute the area of the objects fed into the pipeline.
pub struct Measure<T>
where
    T: CoordFloat,
{
    mode: MeasureMode,
    length_sum: T,
    p00: Coordinate<T>,
    p0: Coordinate<T>,
    #[derivative(Debug = "ignore")]
    point_fn: fn(&mut Self, &Coordinate<T>),
}

// Ignore the state machine functions.
impl<T> PartialEq for Measure<T>
where
    T: CoordFloat,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.length_sum == other.length_sum && self.p00 == other.p00 && self.p0 == other.p0
    }
}

impl<T> Default for Measure<T>
where
    T: AddAssign + CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            // in the javascript, this is lengthRing
            mode: MeasureMode::None,
            length_sum: T::zero(),
            p0: Coordinate {
                x: T::nan(),
                y: T::nan(),
            },
            p00: Coordinate {
                x: T::nan(),
                y: T::nan(),
            },
            point_fn: Self::point_noop,
        }
    }
}

impl<T> Measure<T>
where
    T: AddAssign + CoordFloat,
{
    #[inline]
    #[allow(clippy::unused_self)]
    fn point_noop(&mut self, _p: &Coordinate<T>) {}

    fn length_point_first(&mut self, p: &Coordinate<T>) {
        self.point_fn = Self::length_point;
        self.p0 = *p;
        self.p00 = *p;
    }

    fn length_point(&mut self, p: &Coordinate<T>) {
        self.p0 = self.p0 - *p;

        self.length_sum += (self.p0.x * self.p0.x + self.p0.y * self.p0.y).sqrt();
        self.p0 = *p;
    }
}

impl<T> Result for Measure<T>
where
    T: CoordFloat,
{
    type Out = T;

    fn result(&mut self) -> Self::Out {
        let length = self.length_sum;
        self.length_sum = T::zero();
        length
    }
}

impl<T> Stream for Measure<T>
where
    T: AddAssign + CoordFloat + FloatConst,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
        self
    }

    #[inline]
    fn line_end(&mut self) {
        if let MeasureMode::Polygon = self.mode {
            self.length_point(&self.p00.clone());
        };

        self.point_fn = Self::point_noop;
    }

    #[inline]
    fn line_start(&mut self) {
        self.point_fn = Self::length_point_first;
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        (self.point_fn)(self, p);
    }

    fn polygon_end(&mut self) {
        self.mode = MeasureMode::None;
    }
    fn polygon_start(&mut self) {
        self.mode = MeasureMode::Polygon;
    }
}
