use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::Build;
use crate::projection::ClipExtentAdjust;
use crate::projection::RotateGet;
use crate::projection::ScaleGet;
use crate::projection::TransformExtent;
use crate::rot::rotate_radians;
use crate::Coordinate;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> Reclip
    for Builder<DRAIN, I, LB, LC, LU, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
    DRAIN: Clone,
    I: Clone,
    LC: Clone,
    LU: Clone,
    PV: Clone,
    RC: Clone,
    RU: Clone,
    PV: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    fn reclip(mut self) -> Self {
        let k = T::PI() * self.base.scale();

        let rotate_raw = self.base.rotate();
        let t = rotate_radians(rotate_raw).invert(&Coordinate {
            x: T::zero(),
            y: T::zero(),
        });
        let t = self.base.build().transform(&t);
        let ce = match self.extent {
            Some(extent) => {
                // MercatorRaw and MercatorTransverseRaw supply different
                // transforms
                // todo!("must change transform based on PR");
                // but for now assume projectionMercator is being used.
                self.pr.clone().transform_extent(
                    k,
                    t,
                    extent[0].x,
                    extent[0].y,
                    extent[1].x,
                    extent[1].y,
                )
            }
            _ => [
                Coordinate {
                    x: t.x - k,
                    y: t.y - k,
                },
                Coordinate {
                    x: t.x + k,
                    y: t.y + k,
                },
            ],
        };
        self.base = self.base.clip_extent_adjust(&ce);
        self
    }
}
