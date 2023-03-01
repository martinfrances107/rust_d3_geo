use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::ClipExtentAdjust;
use crate::Transform;

use crate::path::bounds::Bounds;

use super::types::BuilderConicAntimeridianResampleClip;
use super::types::BuilderConicAntimeridianResampleNoneClip;
use super::PRConic;

impl<PR, T> ClipExtentAdjust for BuilderConicAntimeridianResampleClip<Bounds<T>, PR, T>
where
    PR: Clone + PRConic<T = T> + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    fn clip_extent_adjust(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.base.clip_extent_adjust(extent);
        self
    }
}

impl<PR, T> ClipExtentAdjust for BuilderConicAntimeridianResampleNoneClip<Bounds<T>, PR, T>
where
    PR: Clone + Debug + PRConic<T = T> + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    fn clip_extent_adjust(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.base.clip_extent_adjust(extent);
        self
    }
}
