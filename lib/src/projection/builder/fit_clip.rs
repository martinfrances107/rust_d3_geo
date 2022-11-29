use std::fmt::Debug;

use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::path::bounds::Bounds;
use crate::projection::fit_clip::fit_extent_clip;
use crate::projection::fit_clip::fit_height_clip;
use crate::projection::fit_clip::fit_size_clip;
use crate::projection::fit_clip::fit_width_clip;
use crate::projection::Fit;
use crate::stream::Streamable;
use crate::Transform;

use super::types::BuilderAntimeridianResampleClip;

impl<PR, T> Fit for BuilderAntimeridianResampleClip<Bounds<T>, PR, T>
where
    PR: Clone + Debug + Transform<T = T>,
    T: 'static + CoordFloat + Default + FloatConst,
{
    type T = T;

    #[inline]
    fn fit_extent(
        &mut self,
        extent: [Coord<T>; 2],
        object: &impl Streamable<T = Self::T>,
    ) -> &mut Self {
        fit_extent_clip(self, extent, object);
        self
    }

    #[inline]
    fn fit_size(&mut self, size: Coord<T>, object: &impl Streamable<T = T>) -> &mut Self {
        fit_size_clip(self, size, object);
        self
    }

    #[inline]
    fn fit_width(&mut self, w: T, object: &impl Streamable<T = T>) -> &mut Self {
        fit_width_clip(self, w, object);
        self
    }

    #[inline]
    fn fit_height(&mut self, h: T, object: &impl Streamable<T = T>) -> &mut Self {
        fit_height_clip(self, h, object);
        self
    }
}
