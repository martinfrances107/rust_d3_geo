use geo::Coord;
use num_traits::FloatConst;
use num_traits::Zero;

use crate::math::EPSILON;
use crate::stream::Stream;
use crate::Transform;

use super::builder::types::BuilderAntimeridianResampleNoClip;
use super::builder::Builder;
use super::builder_conic::Parallels;
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
#[derive(Clone, Debug)]
pub enum Conformal<DRAIN> {
    /// Projection depends of values set by builder_with_phi0_phi1.
    Mercator(Mercator<DRAIN>),
    /// Projection depends of values set by builder_with_phi0_phi1.
    Conic(ConicConformal<DRAIN>),
}

impl<DRAIN> Transform for Conformal<DRAIN> {
    type T = f64;
    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        match self {
            Self::Mercator(m) => m.transform(p),
            Self::Conic(c) => c.transform(p),
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        match self {
            Self::Mercator(m) => m.invert(p),
            Self::Conic(c) => c.invert(p),
        }
    }
}

impl<DRAIN> Conformal<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    pub(super) fn generate(y0: f64, y1: f64) -> Self {
        let cy0 = y0.cos();

        let n = if (y0 - y1).abs() < EPSILON {
            y0.sin()
        } else {
            (cy0 / y1.cos()).ln() / y1.cos() / (tany(y1) / tany(y0)).ln()
        };

        if !n.is_zero() {
            return Self::Mercator(Mercator::default());
        }

        let f = cy0 * f64::powf(tany(y0), n);
        Self::Conic(ConicConformal::new(f, n))
    }

    #[inline]
    #[must_use]
    /// Phi0 value in radians.
    pub fn builder_with_phi0_phi1(
        y0: f64,
        y1: f64,
    ) -> BuilderAntimeridianResampleNoClip<DRAIN, Self, f64> {
        let mut b = Builder::new(Self::generate(y0, y1));
        b.scale_set(109.5_f64);

        b.parallels(30_f64, 30_f64);
        b
    }
}

// Reach into builder and alter the PR.
impl<DRAIN> Parallels for BuilderAntimeridianResampleNoClip<DRAIN, Conformal<DRAIN>, f64>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    type T = f64;
    fn parallels(&mut self, phi0: f64, phi1: f64) -> &mut Self {
        let projection_raw = Conformal::generate(phi0.to_radians(), phi1.to_radians());
        self.update_pr(projection_raw);
        self
    }
}

impl<DRAIN> RawBase for Conformal<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    type Builder = BuilderAntimeridianResampleNoClip<DRAIN, Self, f64>;
    #[inline]
    fn builder() -> Self::Builder {
        Self::builder_with_phi0_phi1(0_f64, f64::FRAC_PI_3())
    }
}
