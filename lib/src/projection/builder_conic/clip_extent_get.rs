use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::path::bounds::Bounds;
use crate::projection::ClipExtentGet;
use crate::projection::TransformExtent;
use crate::Transform;

use super::types::BuilderConicAntimeridianResampleClip;
use super::types::BuilderConicAntimeridianResampleNoneClip;
use super::PRConic;

impl<PR, T> ClipExtentGet for BuilderConicAntimeridianResampleClip<Bounds<T>, PR, T>
where
    PR: Clone + PRConic<T = T> + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    /// Returns a bounding box.
    #[inline]
    fn clip_extent(&self) -> [Coord<Self::T>; 2] {
        self.base.clip_extent()
    }
}

impl<PR, T> ClipExtentGet for BuilderConicAntimeridianResampleNoneClip<Bounds<T>, PR, T>
where
    PR: Clone + Debug + PRConic<T = T> + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    /// Returns a bounding box.
    #[inline]
    fn clip_extent(&self) -> [Coord<Self::T>; 2] {
        self.base.clip_extent()
    }
}
