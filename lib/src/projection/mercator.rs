//! Unlike all other raw projections `Mercator` and `MercatorTransverse` are
//! hard coded to work only with f64s The Additional dynamic range/
//! resolution  is essential in giving accuarate results near the poles.
use core::fmt::Debug;

use geo_types::Coord;
use num_traits::float::FloatConst;

use crate::projection::ScaleSet;
use crate::Transform;

use super::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use super::builder_mercator::Builder as MercatorBuilder;
use super::RawBase;
use super::TransformExtent;

/// Projection definition. ``Mercator::builder()`` returns a builder.
#[derive(Clone, Debug, Default)]
pub struct Mercator {}

impl RawBase for Mercator {
    type Builder<DRAIN: Clone> = BuilderMercatorAntimeridianResampleClip<DRAIN, Self, f64>;

    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        let mut default: Self::Builder<DRAIN> = MercatorBuilder::new(Self {});
        default.scale_set(961_f64 / f64::TAU());
        default
    }
}

impl TransformExtent for Mercator {
    type T = f64;
    #[inline]
    fn transform_extent(
        &self,
        k: f64,
        t: Coord<f64>,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> [Coord<f64>; 2] {
        [
            Coord {
                x: f64::max(t.x - k, x0),
                y: y0,
            },
            Coord {
                x: f64::min(t.x + k, x1),
                y: y1,
            },
        ]
    }
}

impl Transform for Mercator {
    type T = f64;

    #[inline]
    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        // Divergence between f64 and f32
        // when p.y  = 1.5707963267948966  (PI/2)
        // f64 outputs -37.33185619326892 which is consistent
        // with JS.
        // The f32 is different from JS. Technically
        // tan(PI/2) is NAN. and so log(NAN) is NAN.
        // The value returned
        // from tan(PI_f64/2_f64) happens to be the same
        // large number in both the JS and RUST.
        Coord {
            x: p.x,
            y: ((f64::FRAC_PI_2() + p.y) / 2f64).tan().ln(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        Coord {
            x: p.x,
            y: 2f64.mul_add((p.y.exp()).atan(), -f64::FRAC_PI_2()),
        }
    }
}
