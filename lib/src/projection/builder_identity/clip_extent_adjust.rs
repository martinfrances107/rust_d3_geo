use geo::CoordFloat;
use geo_types::Coord;

use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::PCNU;
use crate::projection::ClipExtentAdjust;

use super::Builder;

impl<DRAIN, T> ClipExtentAdjust for Builder<DRAIN, PCNU<T>, T>
where
    T: CoordFloat,
{
    type T = T;

    fn clip_extent_adjust<CLIPC>(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.postclip = Rectangle::new(extent);

        self
    }
}
