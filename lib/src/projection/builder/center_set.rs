use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::CenterSet;
use crate::projection::Recenter;
use crate::Transform;

use super::template::NoPCNU;
use super::template::ResampleNoPCNC;
use super::template::ResampleNonePCNC;
use super::template::ResampleNonePCNU;
use super::template::ResamplePCNC;
use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<T>,
        PR,
        ResamplePCNC<DRAIN, PR, T>,
        ResamplePCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
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
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU,
        PR,
        ResampleNoPCNC<DRAIN, PR, T>,
        ResampleNoPCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
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
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<T>,
        PR,
        ResampleNonePCNC<DRAIN, PR, T>,
        ResampleNonePCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
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
