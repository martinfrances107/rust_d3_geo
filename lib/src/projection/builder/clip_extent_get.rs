use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::ClipExtentGet;

use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, RU, T> ClipExtentGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, RU, T>
where
    DRAIN: Clone,
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    fn clip_extent(&self) -> [Coord<T>; 2] {
        self.postclip.clip_extent()
    }
}
