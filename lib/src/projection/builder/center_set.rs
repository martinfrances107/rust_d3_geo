use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::CenterSet;
use crate::projection::Recenter;
use crate::Transform;

use super::template::NoPCNU;
use super::template::ResampleNonePCNU;
use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center_set(&mut self, p: &Coord<T>) -> &mut Self {
        self.lambda = (p.x % self.t360).to_radians();
        self.phi = (p.y % self.t360).to_radians();
        self.recenter()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
    for Builder<CLIPC, CLIPU, DRAIN, NoPCNU, PR, ResampleNoPCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center_set(&mut self, p: &Coord<T>) -> &mut Self {
        self.lambda = (p.x % self.t360).to_radians();
        self.phi = (p.y % self.t360).to_radians();
        self.recenter()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, ResampleNonePCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center_set(&mut self, p: &Coord<T>) -> &mut Self {
        self.lambda = (p.x % self.t360).to_radians();
        self.phi = (p.y % self.t360).to_radians();
        self.recenter()
    }
}
