use geo::CoordFloat;
use geo::Coordinate;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LineElem<T: CoordFloat> {
    pub p: Coordinate<T>,
    pub m: Option<u8>,
}
