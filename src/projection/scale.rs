// use std::fmt::Display;
// use std::ops::AddAssign;

// use geo::CoordFloat;

// use crate::Transform;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use super::projection::Projection;
// use super::ProjectionRawTrait;

pub trait Scale
// where
//     PR: ProjectionRawTrait,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
{
    type ST;
    type P;
    fn get_scale(&self) -> Self::ST;
    // /**
    //  * Sets the projection’s scale factor to the specified value and returns the projection.
    //  * The scale factor corresponds linearly to the distance between projected points; however, absolute scale factors are not equivalent across projections.
    //  *
    //  * @param scale Scale factor to be used for the projection; the default scale is projection-specific.
    //  */
    // fn scale(&mut self, scale: &F);
    fn scale(self, scale: Self::ST) -> Self::P;
}
