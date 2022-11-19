//! This should not be constructed directly.
//!
//! instead call
//! [`EqualArea::generate()`](super::equal_area::EqualArea::generate)
//!
//! Common Values(degrees) for phi0
//! 0 - Lambert
//! 30 - Behrmann
//! 45 - Gall–Peters
//!
use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::stream::Stream;
use crate::Transform;

/// Projection definition.
///
/// Why is the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Debug)]
pub struct CylindricalEqualArea<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    cos_phi0: T,
}

impl<DRAIN, T> CylindricalEqualArea<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    #[inline]
    /// Constuctor and Projection base on the angle in radians.
    pub fn new(phi0: T) -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
            cos_phi0: phi0.cos(),
        }
    }
}

impl<DRAIN, T> Transform for CylindricalEqualArea<DRAIN, T>
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
