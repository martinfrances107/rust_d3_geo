// use std::fmt::Display;
// use std::ops::AddAssign;

// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

pub trait PointVisible {
    type PVC;
    fn point_visible(&self, p: &Self::PVC, z: Option<u8>) -> bool;
}
