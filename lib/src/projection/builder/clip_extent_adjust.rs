use geo::CoordFloat;
use geo::Coordinate;

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

    fn clip_extent_adjust(&mut self, extent: &[Coordinate<T>; 2]) -> &mut Self {
        self.x0 = Some(extent[0].x);
        self.y0 = Some(extent[0].y);
        self.x1 = Some(extent[1].x);
        self.y1 = Some(extent[1].y);
        self.postclip = Rectangle::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y);
        self
    }
}
