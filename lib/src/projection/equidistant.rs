use geo::Coord;
use num_traits::Zero;

use crate::math::EPSILON;
use crate::Transform;

use super::builder::types::BuilderAntimeridianResampleNoClip;
use super::builder_conic::Builder;
use super::builder_conic::PRConic;
use super::conic_equidistant::ConicEquidistant;
use super::equirectangular::Equirectangular;
use super::BuilderTrait;
use super::CenterSet;
use super::RawBase;
use super::ScaleSet;

/// Projection definition. ``Equidistant::builder()`` returns a builder.
#[derive(Clone, Debug, Default)]
pub enum Equidistant {
    /// Projection depends of values set by builder_with_phi0_phi1.
    Conic(ConicEquidistant),
    /// Projection depends of values set by builder_with_phi0_phi1.
    Equi(Equirectangular<f64>),
    /// State before the parallels are set.
    #[default]
    Uninitialized,
}

impl Transform for Equidistant {
    type T = f64;
    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        match self {
            Self::Conic(c) => c.transform(p),
            Self::Equi(e) => e.transform(p),
            Self::Uninitialized => Coord {
                x: f64::NAN,
                y: f64::NAN,
            },
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        match self {
            Self::Conic(c) => c.invert(p),
            Self::Equi(e) => e.invert(p),
            Self::Uninitialized => Coord {
                x: f64::NAN,
                y: f64::NAN,
            },
        }
    }
}

impl PRConic for Equidistant {
    fn generate(self, y0: f64, y1: f64) -> Self {
        let cy0 = y0.cos();

        let diff = y1 - y0;
        let n = if diff.abs() < EPSILON {
            y0.sin()
        } else {
            (cy0 - y1.cos()) / diff
        };

        if n.is_zero() {
            return Self::Equi(Equirectangular::default());
        }

        let g = cy0 / n + y0;
        Self::Conic(ConicEquidistant::new(g, n))
    }
}

impl RawBase for Equidistant {
    type Builder<DRAIN: Clone> = Builder<BuilderAntimeridianResampleNoClip<DRAIN, Self, f64>, f64>;
    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        let mut b = Builder::new(Self::default());
        b.scale_set(131.154_f64).center_set(&Coord {
            x: 0_f64,
            y: 13.9389_f64,
        });
        b
    }
}
