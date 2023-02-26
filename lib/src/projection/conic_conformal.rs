use std::marker::PhantomData;

use geo_types::Coord;
use num_traits::FloatConst;

use crate::math::EPSILON;
use crate::Transform;

use super::tany;

/// Projection definition.
///
/// Why is the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Debug)]
pub struct ConicConformal<DRAIN> {
    p_drain: PhantomData<DRAIN>,
    f: f64,
    n: f64,
}

impl<DRAIN> ConicConformal<DRAIN> {
    pub(super) const fn new(f: f64, n: f64) -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
            n,
            f,
        }
    }
}

impl<DRAIN> Transform for ConicConformal<DRAIN> {
    type T = f64;

    #[inline]
    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        let mut y = p.y;
        if self.f > 0f64 {
            if y < -f64::FRAC_PI_2() + EPSILON {
                y = -f64::FRAC_PI_2() + EPSILON;
            } else if y > f64::FRAC_PI_2() - EPSILON {
                y = f64::FRAC_PI_2() - EPSILON;
            }
        }
        let r = self.f / f64::powf(tany(y), self.n);

        Coord {
            x: r * (self.n * p.x).sin(),
            y: r.mul_add(-(self.n * p.x).cos(), self.f),
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        let fy = self.f - p.y;
        let r = self.n.signum() * p.x.hypot(fy);
        let mut l = p.x.atan2(fy.abs()) * fy.signum();
        if fy * self.n < 0f64 {
            l -= f64::PI() * p.x.signum() * fy.signum();
        }

        Coord {
            x: l / self.n,
            y: 2f64.mul_add(
                f64::powf(self.f / r, 1f64 / self.n).atan(),
                -f64::FRAC_PI_2(),
            ),
        }
    }
}
