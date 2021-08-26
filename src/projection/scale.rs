pub trait Scale {
    type T;
    type Builder;

    fn get_scale(&self) -> Self::T;

    // /**
    //  * Sets the projection’s scale factor to the specified value and returns the projection.
    //  * The scale factor corresponds linearly to the distance between projected points; however, absolute scale factors are not equivalent across projections.
    //  *
    //  * @param scale Scale factor to be used for the projection; the default scale is projection-specific.
    //  */
    // fn scale(&mut self, scale: &F);
    fn scale(self, scale: Self::T) -> Self::Builder;
}
