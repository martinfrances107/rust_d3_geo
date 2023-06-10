//! Unlike all other raw projections `Mercator` and `MercatorTransverse` are
//! hard coded to work only with f64s The Additional dynamic range/
//! resolution  is essential in giving accuarate results near the poles.
use core::fmt::Debug;

use geo_types::Coord;
use num_traits::float::FloatConst;

use crate::Transform;

use super::builder_mercator_transverse::types::BuilderMercatorTransverseAntimeridianResampleClip;
use super::builder_mercator_transverse::Builder as MercatorTraverseBuilder;
use super::RawBase;
use super::TransformExtent;

/// Projection definition. ``MercatorTransverse::builder()`` returns a builder.
#[derive(Clone, Debug, Default)]
pub struct MercatorTransverse {}

impl RawBase for MercatorTransverse {
    type Builder<DRAIN: Clone> =
        BuilderMercatorTransverseAntimeridianResampleClip<DRAIN, Self, f64>;

    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        MercatorTraverseBuilder::new(Self::default())
    }
}

impl Transform for MercatorTransverse {
    type T = f64;

    #[inline]
    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        let angle = (f64::FRAC_PI_2() + p.y) / 2f64;
        // Javascript compatibility mode.
        // let tan_angle = if angle == f64::FRAC_PI_2() {
        //     16331239353195370_f64
        // } else if angle == -f64::FRAC_PI_2() {
        //     -16331239353195370_f64
        // } else {
        //     angle.tan()
        // };
        let tan_angle = angle.tan();
        Coord {
            x: tan_angle.ln(),
            y: -p.x,
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        Coord {
            x: -p.y,
            y: 2f64.mul_add((p.x.exp()).atan(), -f64::FRAC_PI_2()),
        }
    }
}

impl TransformExtent for MercatorTransverse {
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
                x: x0,
                y: f64::max(t.y - k, y0),
            },
            Coord {
                x: x1,
                y: f64::min(t.y + k, y1),
            },
        ]
    }
}
