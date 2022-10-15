//! A Raw Projection.
//!
//! Unlike all other raw projections Mercator and MercatorTransverse are
//! hard coded to work only with f64s The Additional dynamic range/
//! resolution  is essential in giving accuarate results near the poles.
use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::projection::ScaleSet;
use crate::stream::Stream;
use crate::Transform;

use super::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use super::builder_mercator::Builder as MercatorBuilder;
use super::ProjectionRawBase;
use super::TransformExtent;

/// Projection definition.
#[derive(Clone, Copy, Debug)]
pub struct Mercator<DRAIN> {
    p_drain: PhantomData<DRAIN>,
}

impl<DRAIN> Default for Mercator<DRAIN> {
    fn default() -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
        }
    }
}

impl<DRAIN> ProjectionRawBase for Mercator<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    type Builder = BuilderMercatorAntimeridianResampleClip<DRAIN, Mercator<DRAIN>, f64>;

    #[inline]
    fn builder() -> Self::Builder {
        let mut default: BuilderMercatorAntimeridianResampleClip<DRAIN, Mercator<DRAIN>, f64> =
            MercatorBuilder::new(Mercator::default());
        let default = default.scale_set(961_f64 / f64::TAU());
        default.to_owned()
    }
}

impl<DRAIN> TransformExtent for Mercator<DRAIN> {
    type T = f64;
    #[inline]
    fn transform_extent(
        self,
        k: f64,
        t: Coordinate<f64>,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> [Coordinate<f64>; 2] {
        [
            Coordinate {
                x: f64::max(t.x - k, x0),
                y: y0,
            },
            Coordinate {
                x: f64::min(t.x + k, x1),
                y: y1,
            },
        ]
    }
}

impl<DRAIN> Transform for Mercator<DRAIN> {
    type T = f64;

    #[inline]
    fn transform(&self, p: &Coordinate<f64>) -> Coordinate<f64> {
        // Divergence between f64 and f32
        // when p.y  = 1.5707963267948966  (PI/2)
        // f64 outputs -37.33185619326892 which is consistent
        // with JS.
        // The f32 is different from JS. Technically
        // tan(PI/2) is NAN. and so log(NAN) is NAN.
        // The value returned
        // from tan(PI_f64/2_f64) happens to be the same
        // large number in both the JS and RUST.
        Coordinate {
            x: p.x,
            y: ((f64::FRAC_PI_2() + p.y) / 2f64).tan().ln(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<f64>) -> Coordinate<f64> {
        Coordinate {
            x: p.x,
            y: 2f64 * (p.y.exp()).atan() - f64::FRAC_PI_2(),
        }
    }
}
