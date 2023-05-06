use geo::Coord;
use num_traits::Zero;

use crate::math::EPSILON;
use crate::stream::Stream;
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

/// Projection definition.
///
/// Why is the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Debug, Default)]
pub enum Equidistant<DRAIN> {
    /// Projection depends of values set by builder_with_phi0_phi1.
    Conic(ConicEquidistant<DRAIN>),
    /// Projection depends of values set by builder_with_phi0_phi1.
    Equi(Equirectangular<DRAIN, f64>),
    /// State before the parallels are set.
    #[default]
    Uninitialized,
}

impl<DRAIN> Transform for Equidistant<DRAIN> {
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

impl<DRAIN> PRConic for Equidistant<DRAIN>
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
            (cy0 - y1.cos()) / (y1 - y0)
        };

        if n.is_zero() {
            return Self::Equi(Equirectangular::default());
        }

        let g = cy0 / n + y0;
        Self::Conic(ConicEquidistant::new(g, n))
    }
}

impl<DRAIN> RawBase for Equidistant<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    type Builder = Builder<BuilderAntimeridianResampleNoClip<DRAIN, Self, f64>, f64>;
    #[inline]
    fn builder() -> Self::Builder {
        let mut b = Builder::new(Self::default());
        b.scale_set(131.154_f64).center_set(&Coord {
            x: 0_f64,
            y: 13.9389_f64,
        });
        b
    }
}
