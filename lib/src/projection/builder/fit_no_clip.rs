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

use super::template::NoPCNU;
use super::template::ResampleNoPCNC;
use super::template::ResampleNoPCNU;
use super::template::ResampleNoneNoPCNC;
use super::template::ResampleNoneNoPCNU;

impl<CLIPC, CLIPU, PR, T> Fit
    for Builder<
        CLIPC,
        CLIPU,
        Bounds<T>,
        NoPCNU<Bounds<T>>,
        PR,
        ResampleNoneNoPCNC<Bounds<T>, PR, T>,
        ResampleNoneNoPCNU<Bounds<T>, PR, T>,
        T,
    >
where
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + Connectable<Output = CLIPC, SC = ResampleNoneNoPCNC<Bounds<T>, PR, T>>,
    PR: Clone + Debug + Transform<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn fit_extent(
        &mut self,
        extent: [[T; 2]; 2],
        object: &impl Streamable<T = Self::T>,
    ) -> &mut Self {
        fit_extent_no_clip(self, extent, object);
        self
    }

    #[inline]
    fn fit_height(&mut self, h: T, object: &impl Streamable<T = T>) -> &mut Self {
        fit_height_no_clip(self, h, object);
        self
    }

    #[inline]
    fn fit_size(&mut self, size: [T; 2], object: &impl Streamable<T = T>) -> &mut Self {
        fit_size_no_clip(self, size, object);
        self
    }
    #[inline]
    fn fit_width(&mut self, w: T, object: &impl Streamable<T = T>) -> &mut Self {
        fit_width_no_clip(self, w, object);
        self
    }
}

impl<CC, CU, PR, T> Fit
    for Builder<
        CC,
        CU,
        Bounds<T>,
        NoPCNU<Bounds<T>>,
        PR,
        ResampleNoPCNC<Bounds<T>, PR, T>,
        ResampleNoPCNU<Bounds<T>, PR, T>,
        T,
    >
where
    CU: Clone + Connectable<Output = CC, SC = ResampleNoPCNC<Bounds<T>, PR, T>>,
    CC: Clone + Stream<EP = Bounds<T>, T = T>,
    PR: Clone + Transform<T = T>,
    PR: Clone + Transform<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn fit_extent(
        &mut self,
        extent: [[T; 2]; 2],
        object: &impl Streamable<T = Self::T>,
    ) -> &mut Self {
        fit_extent_no_clip(self, extent, object);
        self
    }

    #[inline]
    fn fit_size(&mut self, size: [T; 2], object: &impl Streamable<T = T>) -> &mut Self {
        fit_size_no_clip(self, size, object);
        self
    }

    #[inline]
    fn fit_width(&mut self, w: T, object: &impl Streamable<T = T>) -> &mut Self {
        fit_width_no_clip(self, w, object);
        self
    }

    #[inline]
    fn fit_height(&mut self, h: T, object: &impl Streamable<T = T>) -> &mut Self {
        fit_height_no_clip(self, h, object);
        self
    }
}
