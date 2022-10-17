//! A Raw Projection.
//!
//! Unlike all other raw projections Mercator and MercatorTransverse are
//! hard coded to work only with f64s The Additional dynamic range/
//! resolution  is essential in giving accuarate results near the poles.
use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::stream::Stream;
use crate::Transform;

use super::builder_mercator_transverse::types::BuilderMercatorTransverseAntimeridianResampleClip;
use super::builder_mercator_transverse::Builder as MercatorTraverseBuilder;
use super::ProjectionRawBase;
use super::TransformExtent;

/// Projection definition.
#[derive(Clone, Copy, Debug)]
pub struct MercatorTransverse<DRAIN> {
    p_drain: PhantomData<DRAIN>,
}

impl<DRAIN> Default for MercatorTransverse<DRAIN> {
    fn default() -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
        }
    }
}

impl<DRAIN> ProjectionRawBase for MercatorTransverse<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    type Builder =
        BuilderMercatorTransverseAntimeridianResampleClip<DRAIN, MercatorTransverse<DRAIN>, f64>;

    #[inline]
    fn builder() -> Self::Builder {
        MercatorTraverseBuilder::new(MercatorTransverse::default())
    }
}

impl<DRAIN> Transform for MercatorTransverse<DRAIN> {
    type T = f64;

    #[inline]
    fn transform(&self, p: &Coordinate<f64>) -> Coordinate<f64> {
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
        Coordinate {
            x: tan_angle.ln(),
            y: -p.x,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<f64>) -> Coordinate<f64> {
        Coordinate {
            x: -p.y,
            y: 2f64 * (p.x.exp()).atan() - f64::FRAC_PI_2(),
        }
    }
}

impl<DRAIN> TransformExtent for MercatorTransverse<DRAIN> {
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
                x: x0,
                y: f64::max(t.x - k, y0),
            },
            Coordinate {
                x: x1,
                y: f64::min(t.y + k, y1),
            },
        ]
    }
}
