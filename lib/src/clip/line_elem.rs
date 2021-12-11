use geo::CoordFloat;
use geo::Coordinate;

/// Allows the stoage of messages related the the state of a line segment.???
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineElem<T: CoordFloat> {
    /// A 2-D coordinate.
    pub p: Coordinate<T>,
    /// Message assocoated with the point.
    pub m: Option<u8>,
}
