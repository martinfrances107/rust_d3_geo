use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;

use crate::clip::PointVisible;

/// Antimeridian PV ( Point Visible).
#[derive(Clone, Debug, Default)]
pub struct PV<T> {
    /// PhantomData:
    /// The hidden linkage is than any state stored here
    /// must be of the type T from the PointVisible trait.
    ///
    /// The is no actual state stored here but there is
    /// in circle/PV and so to be consitent this MUST
    /// be included here.
    pd: PhantomData<T>,
}

impl<T> PointVisible for PV<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn point_visible(&self, _p: &Coord<T>) -> bool {
        true
    }
}
