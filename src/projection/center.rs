use geo::CoordFloat;
use geo::Coordinate;

pub trait Center // where
//     T: CoordFloat,
{
    type T;

    /**
     * Returns the current center of the projection, which defaults to ⟨0°,0°⟩.
     */
    fn get_center(&self) -> Coordinate<Self::T>
    where
        Self::T: CoordFloat;

    /**
     * Sets the projection’s center to the specified center,
     * a two-element array of longitude and latitude in degrees and returns the projection.
     * The default is ⟨0°,0°⟩.
     *
     * @param point A point specified as a two-dimensional array [longitude, latitude] in degrees.
     */
    fn center(self, point: Coordinate<Self::T>) -> Self
    where
        Self::T: CoordFloat;
}
