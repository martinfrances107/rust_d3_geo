use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipU;
use crate::projection::ClipExtentSet;

use super::Builder;

impl<DRAIN, T> ClipExtentSet for Builder<DRAIN, NoClipU<DRAIN>, T>
where
    DRAIN: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type T = T;
    type Output = Builder<DRAIN, ClipU<DRAIN, T>, T>;

    #[inline]
    fn clip_extent_set(self, extent: &[Coordinate<Self::T>; 2]) -> Self::Output {
        Self::Output {
            p_drain: self.p_drain,

            alpha: self.alpha,
            k: self.k,
            sx: self.sx,
            sy: self.sy,
            ca: self.ca,
            sa: self.sa,
            kx: self.kx,
            ky: self.ky,
            tx: self.tx,
            ty: self.ty,

            // Mutate section.
            postclip: Rectangle::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y),
            x0: None,
            y0: None,
            x1: None,
            y1: None,
        }
        .reset()
    }
}
