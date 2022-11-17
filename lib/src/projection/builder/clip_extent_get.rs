use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::ClipExtentGet;

use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, RC, RU, T> ClipExtentGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    fn clip_extent(&self) -> Option<[Coord<T>; 2]> {
        match (self.x0, self.y0, self.x1, self.y1) {
            (Some(x0), Some(y0), Some(x1), Some(y1)) => {
                Some([Coord { x: x0, y: y0 }, Coord { x: x1, y: y1 }])
            }
            _ => None,
        }
    }
}
