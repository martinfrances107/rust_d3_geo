use geo::CoordFloat;
use geo::Coordinate;

use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::PCNU;
use crate::projection::ClipExtentAdjust;

use super::Builder;

impl<DRAIN, T> ClipExtentAdjust for Builder<DRAIN, PCNU<DRAIN, T>, T>
where
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
