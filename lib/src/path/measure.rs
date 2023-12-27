use core::fmt::Debug;
use core::ops::AddAssign;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::Result;

#[derive(Clone, Debug)]
enum MeasureMode {
    None,
    Polygon,
}

#[derive(Clone)]

/// Stream Endpoint: Compute the area of the objects stream on the path.
pub struct Measure<T>
where
    T: CoordFloat,
{
    mode: MeasureMode,
    length_sum: T,
    p00: Coord<T>,
    p0: Coord<T>,
    point_fn: fn(&mut Self, &Coord<T>),
}

impl<T> Debug for Measure<T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Measure<T>")
            .field(&self.mode)
            .field(&self.length_sum)
            .field(&self.p00)
            .field(&self.p0)
            .finish()
    }
}
// Ignore the state machine functions.
impl<T> PartialEq for Measure<T>
where
    T: CoordFloat,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.length_sum == other.length_sum
            && self.p00 == other.p00
            && self.p0 == other.p0
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
            p0: Coord {
                x: T::nan(),
                y: T::nan(),
            },
            p00: Coord {
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
    fn point_noop(&mut self, _p: &Coord<T>) {}

    fn length_point_first(&mut self, p: &Coord<T>) {
        self.point_fn = Self::length_point;
        self.p0 = *p;
        self.p00 = *p;
    }

    fn length_point(&mut self, p: &Coord<T>) {
        self.p0 = self.p0 - *p;

        self.length_sum +=
            (self.p0.x * self.p0.x + self.p0.y * self.p0.y).sqrt();
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
        if matches!(self.mode, MeasureMode::Polygon) {
            self.length_point(&self.p00.clone());
        };

        self.point_fn = Self::point_noop;
    }

    #[inline]
    fn line_start(&mut self) {
        self.point_fn = Self::length_point_first;
    }

    #[inline]
    fn point(&mut self, p: &Coord<T>, _m: Option<u8>) {
        (self.point_fn)(self, p);
    }

    fn polygon_end(&mut self) {
        self.mode = MeasureMode::None;
    }
    fn polygon_start(&mut self) {
        self.mode = MeasureMode::Polygon;
    }
}
