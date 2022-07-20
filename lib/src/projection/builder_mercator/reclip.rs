use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoClip;
use crate::projection::Build;
use crate::projection::ClipExtentSet;
use crate::projection::RotateGet;
use crate::projection::ScaleGet;
use crate::projection::TransformExtent;
use crate::rot::rotate_radians;
use crate::Coordinate;
use crate::Transform;

use super::types::BuilderMercatorAntimeridianResampleNoneClip;
use super::Reclip;

impl<DRAIN, PR, T> Reclip for BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>;

    fn reclip(self) -> Self::Output {
        let k = T::PI() * self.base.get_scale();

        let rotate_raw = self.base.get_rotate();
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
        let base = self.base;
        Self::Output {
            base,
            // base: self.base.clip_extent(&ce),
            pr: self.pr,
            extent: Some(ce),
        }
    }
}

impl<DRAIN, PR, T> Reclip for BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>;

    fn reclip(self) -> Self::Output {
        let k = T::PI() * self.base.get_scale();

        let rotate_raw = self.base.get_rotate();
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

        Self::Output {
            base: self.base.clip_extent(&ce),
            pr: self.pr,
            extent: Some(ce),
        }
    }
}
