//! This should not be constructed directly.
//!
//! instead call `EqualArea::generate`
//!
//! Common Values(degrees) for phi0
//! 0 - Lambert
//! 30 - Behrmann
//! 45 - Gall–Peters
//!

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::Transform;

/// Projection definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CylindricalEqualArea<T> {
    cos_phi0: T,
}

impl<T> CylindricalEqualArea<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    /// Constuctor and Projection based on the angle in radians.
    pub fn new(phi0: T) -> Self {
        Self {
            cos_phi0: phi0.cos(),
        }
    }
}

impl<T> Transform for CylindricalEqualArea<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        Coord {
            x: p.x * self.cos_phi0,
            y: p.y.sin() / self.cos_phi0,
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        Coord {
            x: p.x / self.cos_phi0,
            y: (p.y * self.cos_phi0).asin(),
        }
    }
}
