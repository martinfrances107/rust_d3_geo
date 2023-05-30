use core::fmt::Debug;

use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::path::bounds::Bounds;
use crate::projection::fit_reclip::fit_extent_reclip;
use crate::projection::fit_reclip::fit_height_reclip;
use crate::projection::fit_reclip::fit_size_reclip;
use crate::projection::fit_reclip::fit_width_reclip;
use crate::projection::Fit;
use crate::projection::TransformExtent;
use crate::stream::Streamable;
use crate::Transform;

use super::types::BuilderMercatorAntimeridianResampleClip;
use super::types::BuilderMercatorAntimeridianResampleNoneClip;

impl<PR, T> Fit for BuilderMercatorAntimeridianResampleClip<Bounds<T>, PR, T>
where
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    #[inline]
    fn fit_extent(&self, extent: [Coord<T>; 2], object: &impl Streamable<T = Self::T>) -> Self {
        fit_extent_reclip(self, extent, object)
    }

    #[inline]
    fn fit_size(&self, size: Coord<T>, object: &impl Streamable<T = T>) -> Self {
        fit_size_reclip(self, size, object)
    }

    #[inline]
    fn fit_width(&self, w: T, object: &impl Streamable<T = T>) -> Self {
        fit_width_reclip(self, w, object)
    }

    #[inline]
    fn fit_height(&self, h: T, object: &impl Streamable<T = T>) -> Self {
        fit_height_reclip(self, h, object)
    }
}

impl<PR, T> Fit for BuilderMercatorAntimeridianResampleNoneClip<Bounds<T>, PR, T>
where
    PR: Clone + Debug + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    #[inline]
    fn fit_extent(&self, extent: [Coord<T>; 2], object: &impl Streamable<T = Self::T>) -> Self {
        fit_extent_reclip(self, extent, object)
    }

    #[inline]
    fn fit_size(&self, size: Coord<T>, object: &impl Streamable<T = T>) -> Self {
        fit_size_reclip(self, size, object)
    }

    #[inline]
    fn fit_width(&self, w: T, object: &impl Streamable<T = T>) -> Self {
        fit_width_reclip(self, w, object)
    }

    #[inline]
    fn fit_height(&self, h: T, object: &impl Streamable<T = T>) -> Self {
        fit_height_reclip(self, h, object)
    }
}
