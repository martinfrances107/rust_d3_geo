use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::path::bounds::Bounds;
use crate::projection::fit_reclip::fit_extent_reclip;
use crate::projection::fit_reclip::fit_height_reclip;
use crate::projection::fit_reclip::fit_size_reclip;
use crate::projection::fit_reclip::fit_width_reclip;

use crate::projection::fit_no_clip::fit_extent_no_clip;
use crate::projection::fit_no_clip::fit_height_no_clip;
use crate::projection::fit_no_clip::fit_size_no_clip;
use crate::projection::fit_no_clip::fit_width_no_clip;
use crate::projection::Fit;
use crate::stream::Streamable;
use crate::Transform;

use super::types::BuilderConicAntimeridianResampleClip;
use super::types::BuilderConicAntimeridianResampleNoClip;
use super::types::BuilderConicAntimeridianResampleNoneClip;

use super::PRConic;

impl<PR, T> Fit for BuilderConicAntimeridianResampleClip<Bounds<T>, PR, T>
where
    PR: Clone + PRConic<T = T> + Transform<T = T>,
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

impl<PR, T> Fit for BuilderConicAntimeridianResampleNoneClip<Bounds<T>, PR, T>
where
    PR: Clone + Debug + PRConic<T = T> + Transform<T = T>,
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

impl<PR, T> Fit for BuilderConicAntimeridianResampleNoClip<Bounds<T>, PR, T>
where
    PR: Clone + PRConic<T = T> + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    #[inline]
    fn fit_extent(&self, extent: [Coord<T>; 2], object: &impl Streamable<T = Self::T>) -> Self {
        fit_extent_no_clip(self, extent, object)
    }

    #[inline]
    fn fit_size(&self, size: Coord<T>, object: &impl Streamable<T = T>) -> Self {
        fit_size_no_clip(self, size, object)
    }

    #[inline]
    fn fit_width(&self, w: T, object: &impl Streamable<T = T>) -> Self {
        fit_width_no_clip(self, w, object)
    }

    #[inline]
    fn fit_height(&self, h: T, object: &impl Streamable<T = T>) -> Self {
        fit_height_no_clip(self, h, object)
    }
}

// impl<PR, T> Fit for BuilderConicAntimeridianResampleNoneNoClip<Bounds<T>, PR, T>
// where
//     PR: Clone + Debug + PRConic<T = T> + Transform<T = T> + TransformExtent<T = T>,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     /// f32 or f64
//     type T = T;

//     #[inline]
//     fn fit_extent(&self, extent: [Coord<T>; 2], object: &impl Streamable<T = Self::T>) -> Self {
//         fit_extent_reclip(self, extent, object)
//     }

//     #[inline]
//     fn fit_size(&self, size: Coord<T>, object: &impl Streamable<T = T>) -> Self {
//         fit_size_reclip(self, size, object)
//     }

//     #[inline]
//     fn fit_width(&self, w: T, object: &impl Streamable<T = T>) -> Self {
//         fit_width_reclip(self, w, object)
//     }

//     #[inline]
//     fn fit_height(&self, h: T, object: &impl Streamable<T = T>) -> Self {
//         fit_height_reclip(self, h, object)
//     }
// }
