use geo::CoordFloat;
use geo::Coordinate;

/// A projection builder sub trait.
pub trait ClipExtent {
    /// f64 or f32
    type T;

    /// Returns a bounding box.
    fn get_clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]>
    where
        Self::T: CoordFloat;

    /// Sets the bounding box.
    fn clip_extent(self, extent: Option<[Coordinate<Self::T>; 2]>) -> Self
    where
        Self::T: CoordFloat;
}
