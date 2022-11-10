use geo::CoordFloat;
use geo::Coordinate;

use crate::stream::Stream;

use super::Result;

/// Stream endpoint: Compute the bounding box.
#[derive(Clone, Debug, PartialEq)]
pub struct Bounds<T>
where
    T: CoordFloat,
{
    p0: Coordinate<T>,
    p1: Coordinate<T>,
}

impl<T> Default for Bounds<T>
where
    T: CoordFloat,
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

impl<T> Result for Bounds<T>
where
    T: CoordFloat,
{
    type Out = [Coordinate<T>; 2];

    /// Return the result, reseting the Bounds.
    fn result(&mut self) -> Self::Out {
        let bounds = [self.p0, self.p1];
        *self = Self::default();
        bounds
    }
}

impl<T> Stream for Bounds<T>
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
    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        if p.x < self.p0.x {
            self.p0.x = p.x;
        }
        if p.x > self.p1.x {
            self.p1.x = p.x;
        }
        if p.y < self.p0.y {
            self.p0.y = p.y;
        }
        if p.y > self.p1.y {
            self.p1.y = p.y;
        }
    }
}
