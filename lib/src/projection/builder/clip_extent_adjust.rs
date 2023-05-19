use geo::CoordFloat;
use geo_types::Coord;

use crate::clip::rectangle::Rectangle;
use crate::projection::ClipExtentAdjust;

use super::template::PCNU;
use super::Builder;

impl<CLIPU, PR, RU, T> ClipExtentAdjust for Builder<CLIPU, PCNU<T>, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    fn clip_extent_adjust<CLIPC>(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.postclip = Rectangle::new(extent);
        self
    }
}
