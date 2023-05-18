use std::fmt::Debug;

use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable;
use crate::path::bounds::Bounds;
use crate::projection::builder::Builder;
use crate::projection::fit_no_clip::fit_extent_no_clip;
use crate::projection::fit_no_clip::fit_height_no_clip;
use crate::projection::fit_no_clip::fit_size_no_clip;
use crate::projection::fit_no_clip::fit_width_no_clip;
use crate::projection::Fit;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

use super::template::NoPCNU;
use super::template::ResampleNoPCNC;
use super::template::ResampleNoPCNU;
use super::template::ResampleNoneNoPCNC;
use super::template::ResampleNoneNoPCNU;

impl<CLIPC, CLIPU, PR, T> Fit for Builder<CLIPC, CLIPU, NoPCNU, PR, ResampleNoneNoPCNU<PR, T>, T>
where
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + Connectable<Output = CLIPC, SC = ResampleNoneNoPCNC<Bounds<T>, PR, T>>,
    PR: Clone + Debug + Transform<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn fit_extent(&self, extent: [Coord<T>; 2], object: &impl Streamable<T = Self::T>) -> Self {
        fit_extent_no_clip(self, extent, object)
    }

    #[inline]
    fn fit_height(&self, h: T, object: &impl Streamable<T = T>) -> Self {
        fit_height_no_clip(self, h, object)
    }

    #[inline]
    fn fit_size(&self, size: Coord<T>, object: &impl Streamable<T = T>) -> Self {
        fit_size_no_clip(self, size, object)
    }
    #[inline]
    fn fit_width(&self, w: T, object: &impl Streamable<T = T>) -> Self {
        fit_width_no_clip(self, w, object)
    }
}

impl<CC, CU, PR, T> Fit for Builder<CC, CU, NoPCNU, PR, ResampleNoPCNU<PR, T>, T>
where
    CU: Clone + Connectable<Output = CC, SC = ResampleNoPCNC<Bounds<T>, PR, T>>,
    CC: Clone + Stream<EP = Bounds<T>, T = T>,
    PR: Clone + Transform<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
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
