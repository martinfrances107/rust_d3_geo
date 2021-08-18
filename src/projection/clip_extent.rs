use geo::CoordFloat;
use geo::Coordinate;

pub trait ClipExtent {
    type T;

    fn get_clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]>
    where
        Self::T: CoordFloat;

    fn clip_extent(self, extent: Option<[Coordinate<Self::T>; 2]>) -> Self
    where
        Self::T: CoordFloat;
}
