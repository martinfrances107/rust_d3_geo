use std::fmt::Debug;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::path::bounds::Bounds;
use crate::projection::builder::Builder;
use crate::projection::fit_no_clip::fit_extent_no_clip;
use crate::projection::fit_no_clip::fit_height_no_clip;
use crate::projection::fit_no_clip::fit_size_no_clip;
use crate::projection::fit_no_clip::fit_width_no_clip;
use crate::projection::Fit;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

use super::template::NoClipU;
use super::template::ResampleNoClipC;
use super::template::ResampleNoClipU;
use super::template::ResampleNoneNoClipC;
use super::template::ResampleNoneNoClipU;

impl<CLIPC, CLIPU, PR, T> Fit
    for Builder<
    CLIPC,
        CLIPU,
        Bounds<T>,
        NoClipU<Bounds<T>>,
        PR,
        ResampleNoneNoClipC<Bounds<T>, PR, T>,
        ResampleNoneNoClipU<Bounds<T>, PR, T>,
        T,
    >
where
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + Connectable<Output = CLIPC, SC = ResampleNoneNoClipC<Bounds<T>, PR, T>>,
    PR: Clone + Debug + Transform<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self {
        fit_extent_no_clip(self, extent, object)
    }

    #[inline]
    fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self {
        fit_height_no_clip(self, h, object)
    }

    #[inline]
    fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self {
        fit_size_no_clip(self, size, object)
    }

    #[inline]
    fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self {
        fit_width_no_clip(self, w, object)
    }
}

impl<CC, CU, PR, T> Fit
    for Builder<
    CC,
        CU,
        Bounds<T>,
        NoClipU<Bounds<T>>,
        PR,
        ResampleNoClipC<Bounds<T>, PR, T>,
        ResampleNoClipU<Bounds<T>, PR, T>,
        T,
    >
where
    CU: Clone + Connectable<Output = CC, SC = ResampleNoClipC<Bounds<T>, PR, T>>,
    CC: Clone + Stream<EP = Bounds<T>, T = T>,
    PR: Clone + Transform<T = T>,
    PR: Clone + Transform<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self {
        fit_extent_no_clip(self, extent, object)
    }

    #[inline]
    fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self {
        fit_height_no_clip(self, h, object)
    }

    #[inline]
    fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self {
        fit_size_no_clip(self, size, object)
    }

    #[inline]
    fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self {
        fit_width_no_clip(self, w, object)
    }
}
