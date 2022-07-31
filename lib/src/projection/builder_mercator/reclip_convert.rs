use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::Build;
use crate::projection::ClipExtentSet;
use crate::projection::RotateGet;
use crate::projection::ScaleGet;
use crate::projection::TransformExtent;
use crate::rot::rotate_radians;
use crate::Transform;

use super::types::BuilderMercatorAntimeridianResampleClip;
use super::types::BuilderMercatorAntimeridianResampleNoClip;
use super::ReclipConvert;

impl<DRAIN, PR, T> ReclipConvert for BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst,
{
    type Output = BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>;
    fn reclip_convert(self) -> Self::Output {
        let k = T::PI() * self.scale();

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
        let base = self.base.clip_extent_set(&ce);
        Self::Output {
            pr: self.pr,
            base,
            extent: self.extent,
        }
    }
}
