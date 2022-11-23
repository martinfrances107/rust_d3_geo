use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::ClipExtentGet;

use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, RC, RU, T> ClipExtentGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, RC, RU, T>
where
    DRAIN: Clone,
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    fn clip_extent(&self) -> Option<[Coord<T>; 2]> {
        Some(self.postclip.clip_extent())
    }
}
