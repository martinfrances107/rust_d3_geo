use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::ClipExtentSet;
use crate::stream::Unconnected;

use super::Builder;

impl<T> ClipExtentSet for Builder<Identity<Unconnected>, T>
where
    T: 'static + CoordFloat + Default + FloatConst,
{
    type T = T;
    type Output = Builder<Rectangle<Unconnected, T>, T>;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coord<Self::T>; 2]) -> Self::Output {
        let mut out = Self::Output {
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
            t360: self.t360,

            // Mutate section.
            postclip: Rectangle::new(extent),
        };
        out.reset();
        out
    }
}
