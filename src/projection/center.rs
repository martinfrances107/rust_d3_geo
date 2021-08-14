pub trait Center {
    type C;

    /**
     * Returns the current center of the projection, which defaults to ⟨0°,0°⟩.
     */
    fn get_center(&self) -> Self::C;

    /**
     * Sets the projection’s center to the specified center,
     * a two-element array of longitude and latitude in degrees and returns the projection.
     * The default is ⟨0°,0°⟩.
     *
     * @param point A point specified as a two-dimensional array [longitude, latitude] in degrees.
     */
    fn center(self, point: Self::C) -> Self;
}
