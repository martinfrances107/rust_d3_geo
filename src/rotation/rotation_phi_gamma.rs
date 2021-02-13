// use delaunator::Point;
use geo::{CoordFloat, Coordinate};

use crate::Transform;

#[derive(Clone, Copy, Debug, Default)]
pub struct RotationPhiGamma<T> {
    cos_delta_phi: T,
    sin_delta_phi: T,
    cos_delta_gamma: T,
    sin_delta_gamma: T,
}

impl<T: CoordFloat + 'static> RotationPhiGamma<T> {
    #[inline]
    pub fn new(delta_phi: T, delta_gamma: T) -> Box<dyn Transform<T>> {
        Box::new(Self {
            cos_delta_phi: delta_phi.cos(),
            sin_delta_phi: delta_phi.sin(),
            cos_delta_gamma: delta_gamma.cos(),
            sin_delta_gamma: delta_gamma.sin(),
        })
    }
}

impl<T: CoordFloat> Transform<T> for RotationPhiGamma<T> {
    #[allow(clippy::many_single_char_names)]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let lambda = p.x;
        let phi = p.y;

        let cos_phi = phi.cos();
        let x = lambda.cos() * cos_phi;
        let y = lambda.sin() * cos_phi;
        let z = phi.sin();
        let k = z * self.cos_delta_phi + x * self.sin_delta_phi;

        return Coordinate {
            x: (y * self.cos_delta_gamma - k * self.sin_delta_gamma)
                .atan2(x * self.cos_delta_phi - z * self.sin_delta_phi),
            y: (k * self.cos_delta_gamma + y * self.sin_delta_gamma).asin(),
        };
    }

    #[allow(clippy::many_single_char_names)]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let lambda = p.x;
        let phi = p.y;

        let cos_phi = phi.cos();
        let x = lambda.cos() * cos_phi;
        let y = lambda.sin() * cos_phi;
        let z = phi.sin();
        let k = z * self.cos_delta_gamma - y * self.sin_delta_gamma;

        return Coordinate {
            x: (y * self.cos_delta_gamma + z * self.sin_delta_gamma)
                .atan2(x * self.cos_delta_phi + k * self.sin_delta_phi),
            y: (k * self.cos_delta_phi - x * self.sin_delta_phi).asin(),
        };
    }
}
