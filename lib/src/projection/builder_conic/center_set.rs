use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;
use super::PRConic;

use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::CenterSet;
use crate::Transform;

impl<DRAIN, PR, T> CenterSet for Builder<BuilderAntimeridianResampleNoClip<DRAIN, PR, T>, PR, T>
where
    DRAIN: Clone,
    PR: Clone + PRConic<T = T> + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center_set(&mut self, point: &Coord<Self::T>) -> &mut Self {
        self.base.center_set(point);
        self
    }
}
