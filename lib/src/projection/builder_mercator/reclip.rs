use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::builder::template::PCNU;
use crate::projection::Build;
use crate::projection::ClipExtentAdjust;
use crate::projection::RotateGet;
use crate::projection::ScaleGet;
use crate::projection::TransformExtent;
use crate::rot::rotation::Rotation;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<CLIPC, CLIPU, DRAIN, PR, RC, RU, T> Reclip
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    RU: Clone,
    T: CoordFloat + FloatConst,
{
    fn reclip(&mut self) -> &mut Self {
        let k = T::PI() * self.base.scale();

        let rotate_raw = self.base.rotate();
        let t = Rotation::new(rotate_raw[0], rotate_raw[1], rotate_raw[2]).invert(&Coord {
            x: T::zero(),
            y: T::zero(),
        });
        let t = self.base.build().transform(&t);
        let ce = match self.extent {
            Some(extent) => self.pr.clone().transform_extent(
                k,
                t,
                extent[0].x,
                extent[0].y,
                extent[1].x,
                extent[1].y,
            ),
            _ => [
                Coord {
                    x: t.x - k,
                    y: t.y - k,
                },
                Coord {
                    x: t.x + k,
                    y: t.y + k,
                },
            ],
        };
        self.base.clip_extent_adjust(&ce);
        self
    }
}
