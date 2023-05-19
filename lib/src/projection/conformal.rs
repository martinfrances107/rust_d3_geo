use geo::Coord;
use num_traits::Zero;

use crate::clip::antimeridian::ClipAntimeridianC;
use crate::math::EPSILON;
use crate::Transform;

use super::builder::template::ResampleNoPCNC;
use super::builder::types::BuilderAntimeridianResampleNoClip;
use super::builder_conic::Builder;
use super::builder_conic::PRConic;
use super::builder_conic::ParallelsSet;
use super::conic_conformal::ConicConformal;
use super::mercator::Mercator;
use super::tany;
use super::BuilderTrait;
use super::RawBase;
use super::ScaleSet;

/// Projection definition.
///
/// Why is the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Debug, Default)]
pub enum Conformal {
    /// Projection depends of values set by builder_with_phi0_phi1.
    Conic(ConicConformal),
    /// Projection depends of values set by builder_with_phi0_phi1.
    Mercator(Mercator),
    /// State before the parallels are set.
    #[default]
    Uninitialized,
}

impl Transform for Conformal {
    type T = f64;
    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        match self {
            Self::Mercator(m) => m.transform(p),
            Self::Conic(c) => c.transform(p),
            Self::Uninitialized => Coord {
                x: f64::NAN,
                y: f64::NAN,
            },
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        match self {
            Self::Mercator(m) => m.invert(p),
            Self::Conic(c) => c.invert(p),
            Self::Uninitialized => Coord {
                x: f64::NAN,
                y: f64::NAN,
            },
        }
    }
}

impl PRConic for Conformal {
    fn generate(self, y0: f64, y1: f64) -> Self {
        let cy0 = y0.cos();

        // TODO make optimal after fix.
        #[allow(clippy::suboptimal_flops)]
        let n = if (y0 - y1).abs() < EPSILON {
            y0.sin()
        } else {
            (cy0 / y1.cos()).ln() / (tany(y1) / tany(y0)).ln()
        };

        if n.is_zero() {
            return Self::Mercator(Mercator::default());
        }

        let f = cy0 * f64::powf(tany(y0), n) / n;
        Self::Conic(ConicConformal::new(f, n))
    }
}

impl RawBase for Conformal {
    type Builder<DRAIN: Clone> = Builder<BuilderAntimeridianResampleNoClip<DRAIN, Self, f64>, f64>;
    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        let mut b = Builder::new(Self::default());
        b.scale_set::<ClipAntimeridianC<ResampleNoPCNC<DRAIN, Self, f64>, f64>>(109.5_f64)
            .parallels_set(30_f64, 30_f64);
        b
    }
}
