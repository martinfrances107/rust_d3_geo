use approx::AbsDiffEq;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::ClipExtentAdjust;

use super::Builder;

impl<BASE, T> ClipExtentAdjust for Builder<BASE, T>
where
    BASE: ClipExtentAdjust<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    fn clip_extent_adjust(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.base.clip_extent_adjust(extent);
        self
    }
}
