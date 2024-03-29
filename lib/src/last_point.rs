use geo::CoordFloat;
use geo_types::Coord;

use crate::path::Result;
use crate::stream::Stream;

/// Stream endpoint: Retain the last point.
///
/// This endpoint is used in the `AlbersUSA` projection.
/// If serves as a point mask. The albers has clipping bounds
/// and if a point flows through the pipe line and is retained
/// by `LastPoint` it is in alaska, the lower 48 or in hawaii.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LastPoint<T: CoordFloat>(Option<Coord<T>>);

impl<T> Stream for LastPoint<T>
where
    T: CoordFloat,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn endpoint<'a>(&mut self) -> &mut Self {
        self
    }

    #[inline]
    fn point(&mut self, p: &Coord<T>, _m: Option<u8>) {
        self.0 = Some(*p);
    }
}

impl<T> Result for LastPoint<T>
where
    T: CoordFloat,
{
    type Out = Option<Coord<T>>;

    fn result(&mut self) -> Self::Out {
        let out = self.0;
        self.0 = None;
        out
    }
}
