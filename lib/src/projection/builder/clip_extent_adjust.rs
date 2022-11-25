use geo::CoordFloat;
use geo_types::Coord;

use crate::clip::rectangle::Rectangle;
use crate::projection::ClipExtentAdjust;

use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, RC, RU, T> ClipExtentAdjust
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    fn clip_extent_adjust(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.postclip = Rectangle::new(extent);
        self
    }
}
