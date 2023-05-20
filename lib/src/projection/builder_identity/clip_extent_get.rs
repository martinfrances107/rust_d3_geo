use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::ClipExtentGet;

use super::Builder;

impl<PCNU, T> ClipExtentGet for Builder<PCNU, T>
where
    PCNU: ClipExtentGet<T = T>,
    T: CoordFloat,
{
    type T = T;

    fn clip_extent(&self) -> [Coord<T>; 2] {
        self.postclip.clip_extent()
    }
}
