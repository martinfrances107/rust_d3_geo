use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::PCNU;
use crate::projection::builder_mercator::Reclip;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;

impl<CLIPU, PR, RU, T> Reclip for Builder<CLIPU, PCNU<T>, PR, RU, T>
where
    CLIPU: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    RU: Clone,
    T: CoordFloat + FloatConst,
{
    fn reclip<CLIPC>(&mut self) -> &mut Self {
        self.base.reclip::<CLIPC>();
        self
    }
}
