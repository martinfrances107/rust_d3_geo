use geo::CoordFloat;
use geo_types::Coord;

use crate::clip::rectangle::Rectangle;
use crate::projection::ClipExtentAdjust;
use crate::stream::Unconnected;

use super::Builder;

impl<CLIPU, DRAIN, PR, RU, T> ClipExtentAdjust
    for Builder<CLIPU, DRAIN, Rectangle<Unconnected, T>, PR, RU, T>
where
    T: 'static + CoordFloat,
{
    type T = T;

    fn clip_extent_adjust(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.postclip = Rectangle::new(extent);
        self
    }
}
