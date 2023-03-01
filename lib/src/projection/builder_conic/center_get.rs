use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;
use super::PRConic;

use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::CenterGet;
use crate::Transform;

impl<DRAIN, PR, T> CenterGet for Builder<BuilderAntimeridianResampleNoClip<DRAIN, PR, T>, PR, T>
where
    DRAIN: Clone,
    PR: Clone + PRConic<T = T> + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center(&self) -> Coord<Self::T>
    where
        Self::T: CoordFloat,
    {
        self.base.center()
    }
}
