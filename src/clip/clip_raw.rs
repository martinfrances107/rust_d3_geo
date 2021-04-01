use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use std::ops::AddAssign;

use crate::stream::Stream;

use super::antimeridian::ClipAntimeridian;
use super::circle::ClipCircle;

#[derive(Clone, Debug)]
pub enum ClipRaw<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    Antimeridian(ClipAntimeridian<T>),
    Circle(ClipCircle<T>),
}

impl<T> Default for ClipRaw<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    fn default() -> Self {
        ClipRaw::Antimeridian(ClipAntimeridian::default())
    }
}
