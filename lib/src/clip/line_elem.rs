use geo::CoordFloat;
use geo_types::Coord;

/// Allows the storage of messages related the the state of a line segment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineElem<T>
where
    T: CoordFloat,
{
    /// A 2-D coordinate.
    pub p: Coord<T>,
    /// Message associated with the point.
    pub m: Option<u8>,
}
