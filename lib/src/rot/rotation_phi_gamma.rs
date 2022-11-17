use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::Transform;

/// A rotation is two directions.
#[derive(Clone, Copy, Debug, Default)]
pub struct RotationPhiGamma<T> {
    cos_delta_phi: T,
    sin_delta_phi: T,
    cos_delta_gamma: T,
    sin_delta_gamma: T,
}

impl<'a, T: CoordFloat> RotationPhiGamma<T> {
    /// Constructor.
    #[inline]
    pub fn new(delta_phi: &'a T, delta_gamma: &'a T) -> Self {
        let (sin_delta_phi, cos_delta_phi) = delta_phi.sin_cos();
        let (sin_delta_gamma, cos_delta_gamma) = delta_gamma.sin_cos();
        Self {
            cos_delta_phi,
            sin_delta_phi,
            cos_delta_gamma,
            sin_delta_gamma,
        }
    }
}

impl<T> Transform for RotationPhiGamma<T>
where
    T: CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    #[allow(clippy::many_single_char_names)]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let lambda = p.x;
        let phi = p.y;

        let (sin_phi, cos_phi) = phi.sin_cos();
        let (sin_lambda, cos_lambda) = lambda.sin_cos();
        let x = cos_lambda * cos_phi;
        let y = sin_lambda * cos_phi;
        let z = sin_phi;
        let k = z * self.cos_delta_phi + x * self.sin_delta_phi;

        Coord {
            x: (y * self.cos_delta_gamma - k * self.sin_delta_gamma)
                .atan2(x * self.cos_delta_phi - z * self.sin_delta_phi),
            y: (k * self.cos_delta_gamma + y * self.sin_delta_gamma).asin(),
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        let lambda = p.x;
        let phi = p.y;

        let (sin_lambda, cos_lambda) = lambda.sin_cos();
        let (sin_phi, cos_phi) = phi.sin_cos();
        let x = cos_lambda * cos_phi;
        let y = sin_lambda * cos_phi;
        let z = sin_phi;
        let k = z * self.cos_delta_gamma - y * self.sin_delta_gamma;

        Coord {
            x: (y * self.cos_delta_gamma + z * self.sin_delta_gamma)
                .atan2(x * self.cos_delta_phi + k * self.sin_delta_phi),
            y: (k * self.cos_delta_phi - x * self.sin_delta_phi).asin(),
        }
    }
}
