use geo_types::Coord;

use crate::Transform;

/// Projection definition.
#[derive(Clone, Debug, PartialEq)]
pub struct ConicEquidistant {
    g: f64,
    n: f64,
}

impl ConicEquidistant {
    #[inline]
    pub(super) const fn new(g: f64, n: f64) -> Self {
        Self { g, n }
    }
}

impl Transform for ConicEquidistant {
    type T = f64;

    #[inline]
    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        let gy = self.g - p.y;
        let nx = self.n * p.x;

        Coord {
            x: gy * (nx).sin(),
            y: gy.mul_add(-nx.cos(), self.g),
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        let gy = self.g - p.y;
        let mut l = p.x.atan2(gy.abs()) * gy.signum();

        if (gy * self.n) < 0_f64 {
            l -= std::f64::consts::PI * p.x.signum() * gy.signum();
        }
        Coord {
            x: l / self.n,
            y: self.n.signum().mul_add(-p.x.hypot(gy), self.g),
        }
    }
}
