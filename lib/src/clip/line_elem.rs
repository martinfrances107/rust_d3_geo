use core::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;

/// Allows the storage of messages related the the state of a line segment
///
/// "instead of adding epsilons at the "clipcircle" stage,
/// we pass an explicit message in coordinate z that this point
/// should not be mixed with the previous point, and add the epsilons later,
/// at the "rejoin" stage."
///
/// See <https://github.com/d3/d3-geo/commit/ed81279e6b7f322c7075783410da9da84a6c8c85>
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct LineElem<T>
where
    T: CoordFloat,
{
    /// A 2-D coordinate.
    pub p: Coord<T>,
    /// Message associated with the point.
    /// clipperCircle Algorithm can pass a message to the rejoin
    /// algorithm saying that nudging with a epsilon shhould be delayed.
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
