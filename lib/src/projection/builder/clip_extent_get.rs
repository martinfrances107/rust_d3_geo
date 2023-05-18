use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::ClipExtentGet;

use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, PR, RU, T> ClipExtentGet for Builder<CLIPC, CLIPU, PCNU<T>, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    fn clip_extent(&self) -> [Coord<T>; 2] {
        self.postclip.clip_extent()
    }
}
