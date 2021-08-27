/// Controls the projections scaling factor.
///
/// Projection builder sub trait.
pub trait Scale {
    /// f32 or f64.
    type T;
    /// The output type.
    type Builder;

    /// Returns the programmed scaling factor.
    fn get_scale(&self) -> Self::T;

    ///  Sets the projection’s scale factor to the specified value and returns the projection.
    ///  The scale factor corresponds linearly to the distance between projected points; however, absolute scale factors are not equivalent across projections.
    ///
    ///  @param scale Scale factor to be used for the projection; the default scale is projection-specific.
    fn scale(self, scale: Self::T) -> Self::Builder;
}
