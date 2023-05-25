use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::identity::Identity;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::CenterSet;
use crate::projection::Recenter;
use crate::stream::Unconnected;
use crate::Transform;

use super::template::ResampleNonePCNU;
use super::template::PCNU;
use super::Builder;

impl<CLIPU, DRAIN, PR, T> CenterSet for Builder<CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
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

impl<CLIPU, DRAIN, PR, T> CenterSet
    for Builder<CLIPU, DRAIN, Identity<Unconnected>, PR, ResampleNoPCNU<PR, T>, T>
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

impl<CLIPU, DRAIN, PR, T> CenterSet
    for Builder<CLIPU, DRAIN, PCNU<T>, PR, ResampleNonePCNU<PR, T>, T>
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
