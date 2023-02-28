use geo_types::Coord;
use std::marker::PhantomData;

use crate::Transform;

/// Projection definition.
///
/// Why is the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Debug)]
pub struct ConicEquidistant<DRAIN> {
    p_drain: PhantomData<DRAIN>,
    g: f64,
    n: f64,
}

impl<DRAIN> ConicEquidistant<DRAIN> {
    pub(super) const fn new(g: f64, n: f64) -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
            n,
            g,
        }
    }
}

impl<DRAIN> Transform for ConicEquidistant<DRAIN> {
    type T = f64;

    #[inline]
    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        dbg!("transform");
        dbg!(p);

        let gy = self.g - p.y;
        let nx = self.n * p.x;
        dbg!(gy);
        dbg!(nx);

        Coord {
            x: gy * (nx).sin(),
            y: self.g - gy * nx.cos(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        let gy = self.g - p.y;
        let mut l = p.x.atan2(gy.abs()) * gy.signum();
        dbg!(gy);
        dbg!(l);
        if (gy * self.n) < 0_f64 {
            l -= std::f64::consts::PI * p.x.signum() * gy.signum();
        }
        dbg!(l);
        Coord {
            x: l / self.n,
            y: self.g - self.n.signum() * p.x.hypot(gy),
        }
    }
}
