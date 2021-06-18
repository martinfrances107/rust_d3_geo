// use std::fmt::Display;
// use std::ops::AddAssign;

// use geo::{CoordFloat, Coordinate};

// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use super::projection::Projection;
// use super::ProjectionRawTrait;

pub trait ClipExtent
// where
//     PR: ProjectionRawTrait,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type C;
    type P;
    fn get_clip_extent(&self) -> Option<[Self::C; 2]>;

    fn clip_extent(self, extent: Option<[Self::C; 2]>) -> Self::P;
}
