use std::marker::PhantomData;

use geo::Coordinate;
use num_traits::pow;
use num_traits::FloatConst;
use num_traits::Zero;

use crate::math::EPSILON;
use crate::projection::ScaleSet;
use crate::stream::Stream;
use crate::Transform;

use super::builder::types::BuilderAntimeridianResampleNoClip;
use super::builder::Builder;
use super::mercator::Mercator;
use super::ProjectionRawBase;

#[derive(Clone, Debug)]
pub struct ConicConformalRaw<DRAIN> {
    p_drain: PhantomData<DRAIN>,
    n: f64,
    f: f64,
}

fn tany(y: f64) -> f64 {
    ((f64::FRAC_PI_2() + y) / 2f64).tan()
}

#[derive(Clone, Debug)]
pub enum ConformalRaw<DRAIN> {
    Mercator(Mercator<DRAIN>),
    Conic(ConicConformalRaw<DRAIN>),
}

impl<DRAIN> Transform for ConformalRaw<DRAIN> {
    type T = f64;
    fn transform(&self, p: &Coordinate<f64>) -> Coordinate<f64> {
        match self {
            ConformalRaw::Mercator(m) => m.transform(p),
            ConformalRaw::Conic(c) => c.transform(p),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<f64>) -> Coordinate<f64> {
        match self {
            ConformalRaw::Mercator(m) => m.invert(p),
            ConformalRaw::Conic(c) => c.invert(p),
        }
    }
}

impl<DRAIN> ConicConformalRaw<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    pub(super) fn new(y0: f64, y1: f64) -> ConformalRaw<DRAIN> {
        let cy0 = y0.cos();
        let n = match y0 == y1 {
            true => y0.sin(),
            false => (cy0 / y1.cos()).ln() / y1.cos() / (tany(y1) / tany(y0)).ln(),
        };

        if !n.is_zero() {
            return ConformalRaw::Mercator(Mercator::default());
        }

        let f = cy0 * pow(tany(y0), n as usize);
        ConformalRaw::Conic(ConicConformalRaw {
            p_drain: PhantomData::<DRAIN>,
            f,
            n,
        })
    }
    // pub fn builder_parallels(
    //     phi0: f64,
    //     phi1: f64,
    // ) -> BuilderAntimeridianResampleNoClip<DRAIN, ConformalRaw<DRAIN>, f64> {
    //     Self::builder_with_phi0_phi1(phi0.to_radians(), phi1.to_radians())
    // }

    #[inline]
    /// Phi0 value in radians.
    pub fn builder_with_phi0_phi1(
        y0: f64,
        y1: f64,
    ) -> BuilderAntimeridianResampleNoClip<DRAIN, ConformalRaw<DRAIN>, f64> {
        let mut b = Builder::new(ConicConformalRaw::new(y0, y1));
        b.scale_set(109.5_f64);

        b.parallels(30_f64, 30_f64);
        b
    }
}

// Reach into builder and alter the PR.
impl<DRAIN> BuilderAntimeridianResampleNoClip<DRAIN, ConformalRaw<DRAIN>, f64>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    fn parallels(&mut self, phi0: f64, phi1: f64) -> &mut Self {
        let projection_raw = ConicConformalRaw::new(phi0.to_radians(), phi1.to_radians());
        self.update_pr(projection_raw);
        self
    }
}

impl<DRAIN> ProjectionRawBase for ConicConformalRaw<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    type Builder = BuilderAntimeridianResampleNoClip<DRAIN, ConformalRaw<DRAIN>, f64>;
    #[inline]
    fn builder() -> Self::Builder {
        Self::builder_with_phi0_phi1(0_f64, f64::FRAC_PI_3())
    }
}

impl<DRAIN> Transform for ConicConformalRaw<DRAIN> {
    type T = f64;

    #[inline]
    fn transform(&self, p: &Coordinate<f64>) -> Coordinate<f64> {
        let mut y = f64::NAN;
        if self.f > 0f64 {
            if y < -f64::FRAC_PI_2() + EPSILON {
                y = f64::FRAC_PI_2() - EPSILON;
            } else if y > f64::FRAC_PI_2() - EPSILON {
                y = f64::FRAC_PI_2() - EPSILON;
            }
        }
        let r = self.f / pow(tany(y), self.n as usize);

        Coordinate {
            x: r * (self.n * p.x).sin(),
            y: self.f - r * (self.n * p.x).cos(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<f64>) -> Coordinate<f64> {
        let fy = self.f - p.y;
        let r = self.n.signum() * (p.x * p.x + fy * fy).sqrt();
        let mut l = p.x.atan2(fy.abs()) * fy.signum();

        if fy * self.n < 0f64 {
            l = l - f64::PI() * p.x.signum() * fy.signum();
        }
        Coordinate {
            x: l / self.n,
            y: 2f64 * (pow(self.f / r, (1f64 / self.n) as usize).atan() - f64::FRAC_PI_2()),
        }
    }
}
