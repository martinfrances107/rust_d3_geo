use core::fmt::Debug;

use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::identity::Identity;
use crate::path::bounds::Bounds;
use crate::projection::builder::Builder;
use crate::projection::fit_no_clip::fit_extent_no_clip;
use crate::projection::fit_no_clip::fit_height_no_clip;
use crate::projection::fit_no_clip::fit_size_no_clip;
use crate::projection::fit_no_clip::fit_width_no_clip;
use crate::projection::Fit;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::stream::Unconnected;
use crate::Transform;

use super::template::ResampleNoPCNC;
use super::template::ResampleNoPCNU;
use super::template::ResampleNoneNoPCNC;
use super::template::ResampleNoneNoPCNU;

impl<CLIPC, CLIPU, PR, T> Fit
    for Builder<
        CLIPU,
        Bounds<T>,
        Identity<Unconnected>,
        PR,
        ResampleNoneNoPCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone
        + ConnectableClip<
            Output = CLIPC,
            SC = ResampleNoneNoPCNC<Bounds<T>, PR, T>,
        >,
    PR: Clone + Debug + Transform<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn fit_extent(
        &self,
        extent: [Coord<T>; 2],
        object: &impl Streamable<T = Self::T>,
    ) -> Self {
        fit_extent_no_clip(self, extent, object)
    }

    #[inline]
    fn fit_height(&self, h: T, object: &impl Streamable<T = T>) -> Self {
        fit_height_no_clip(self, h, object)
    }

    #[inline]
    fn fit_size(
        &self,
        size: Coord<T>,
        object: &impl Streamable<T = T>,
    ) -> Self {
        fit_size_no_clip(self, size, object)
    }
    #[inline]
    fn fit_width(&self, w: T, object: &impl Streamable<T = T>) -> Self {
        fit_width_no_clip(self, w, object)
    }
}

impl<CLIPC, CLIPU, PR, T> Fit
    for Builder<
        CLIPU,
        Bounds<T>,
        Identity<Unconnected>,
        PR,
        ResampleNoPCNU<PR, T>,
        T,
    >
where
    CLIPU: Clone
        + ConnectableClip<Output = CLIPC, SC = ResampleNoPCNC<Bounds<T>, PR, T>>,
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    PR: Clone + Transform<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn fit_extent(
        &self,
        extent: [Coord<T>; 2],
        object: &impl Streamable<T = Self::T>,
    ) -> Self {
        fit_extent_no_clip(self, extent, object)
    }

    #[inline]
    fn fit_size(
        &self,
        size: Coord<T>,
        object: &impl Streamable<T = T>,
    ) -> Self {
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
