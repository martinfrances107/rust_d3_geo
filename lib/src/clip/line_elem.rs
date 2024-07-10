use core::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;

/// Allows the storage of messages related the the state of a line segment
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct LineElem<T>
where
    T: CoordFloat,
{
    /// A 2-D coordinate.
    pub p: Coord<T>,
    /// Message associated with the point.
    pub m: Option<u8>,
}

/// Compress output to a single line.
///
/// Useful when debugging a vector of say 100 elements.
impl<T> Debug for LineElem<T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LineElem: {:?} {:?} {:?}", self.p.x, self.p.y, self.m)
    }
}
