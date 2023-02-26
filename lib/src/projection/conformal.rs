use geo::Coord;
use num_traits::Zero;

use crate::math::EPSILON;
use crate::stream::Stream;
use crate::Transform;

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
pub enum Conformal<DRAIN> {
    /// Projection depends of values set by builder_with_phi0_phi1.
    Conic(ConicConformal<DRAIN>),
    /// Projection depends of values set by builder_with_phi0_phi1.
    Mercator(Mercator<DRAIN>),
    /// State before the parallels are set.
    #[default]
    Uninitialized,
}

impl<DRAIN> Transform for Conformal<DRAIN> {
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

impl<DRAIN> PRConic for Conformal<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
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

impl<DRAIN> RawBase for Conformal<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    type Builder = Builder<BuilderAntimeridianResampleNoClip<DRAIN, Self, f64>, Self, f64>;
    #[inline]
    fn builder() -> Self::Builder {
        let mut b = Builder::new(Self::default());

        b.scale_set(109.5_f64).parallels_set(30_f64, 30_f64);
        b
    }
}
