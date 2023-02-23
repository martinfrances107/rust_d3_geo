use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::math::EPSILON;
use crate::projection::BuilderTrait;
use crate::stream::Stream;
use crate::Transform;

use super::builder::types::BuilderAntimeridianResampleNoClip;
use super::builder_conic::Builder;
use super::builder_conic::PRConic;
use super::conic_equal_area::ConicEqualArea;
use super::cylindrical_equal_area::CylindricalEqualArea;
use super::RawBase;

/// [`ConicEqualArea`](super::conic_equal_area::ConicEqualArea) return type.
///
/// Depending construction parameters
/// one of two Projection types are returned.
#[derive(Clone, Debug, Default)]
pub enum EqualArea<DRAIN, T> {
    /// Parallels symetical around the Equator.
    Cyl(CylindricalEqualArea<DRAIN, T>),
    /// Conic
    Con(ConicEqualArea<DRAIN, T>),
    /// State before generator is called.
    #[default]
    Uninitialized,
}
impl<DRAIN, T> Transform for EqualArea<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        match self {
            Self::Cyl(cyl) => cyl.transform(p),
            Self::Con(con) => con.transform(p),
            Self::Uninitialized => Coord {
                x: T::zero(),
                y: T::zero(),
            },
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        match self {
            Self::Cyl(cyl) => cyl.invert(p),
            Self::Con(con) => con.invert(p),
            Self::Uninitialized => Coord {
                x: T::zero(),
                y: T::zero(),
            },
        }
    }
}

impl<DRAIN, T> PRConic for EqualArea<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    /// Inputs select either a conic or a cylindrical projection.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as EPSILON will always be converted into T.
    fn generate(self, y0: T, y1: T) -> Self {
        let two = T::from(2_f64).unwrap();
        let sy0 = y0.sin();
        let n = (sy0 + y1.sin()) / two;

        // Are the parallels symmetrical around the Equator?
        if n.abs() < T::from(EPSILON).unwrap() {
            return Self::Cyl(CylindricalEqualArea::new(y0));
        }
        let c = T::one() + sy0 * (two * n - sy0);
        Self::Con(ConicEqualArea::new(T::one() + sy0, n, c.sqrt() / n, two))
    }
}

//     #[inline]
//     /// Phi0 value in radians.
//     ///
//     /// # Panics
//     /// unwrap() is used here but a panic will never happen as constants will always be converted into T.
//     pub fn builder_with_phi0_phi1(
//         y0: T,
//         y1: T,
//     ) -> Builder<BuilderAntimeridianResampleNoClip<DRAIN, Self, T>, Self, T> {
//         let ea = Self::generate(y0, y1);
//         let mut b: Builder<BuilderAntimeridianResampleNoClip<DRAIN, Self, T>, Self, T> =
//             Builder::new(ea);
//         b.scale_set(T::from(155.424).unwrap()).center_set(&Coord {
//             x: T::zero(),
//             y: T::from(33.6442).unwrap(),
//         });
//         b
//     }
// }

// use crate::projection::builder_conic::PRConic;
// impl<DRAIN, T> PRConic for EqualArea<DRAIN, T>
// where
//     DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
//     T: CoordFloat + Default + FloatConst,
// {
//     fn generate(self, y0: Self::T, y1: Self::T) -> Self {
//         // Self::builder_with_phi0_phi1(T::zero(), T::FRAC_PI_3())
//         todo!();
//     }
// }

impl<DRAIN, T> RawBase for EqualArea<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    // type Builder = BuilderAntimeridianResampleNoClip<DRAIN, Self, T>;
    type Builder = Builder<BuilderAntimeridianResampleNoClip<DRAIN, Self, T>, Self, T>;
    #[inline]
    fn builder() -> Self::Builder {
        Builder::new(Self::default())
    }
}
