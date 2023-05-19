use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianC;
use crate::math::EPSILON;
use crate::Transform;

use super::builder::template::ResampleNoneNoPCNC;
use super::builder::types::BuilderAntimeridianResampleNoClip;
use super::builder_conic::Builder;
use super::builder_conic::PRConic;
use super::conic_equal_area::ConicEqualArea;
use super::cylindrical_equal_area::CylindricalEqualArea;
use super::BuilderTrait;
use super::CenterSet;
use super::RawBase;
use super::ScaleSet;

/// [`ConicEqualArea`](super::conic_equal_area::ConicEqualArea) return type.
///
/// Depending construction parameters
/// one of two Projection types are returned.
#[derive(Clone, Debug, Default)]
pub enum EqualArea<T> {
    /// Parallels symetical around the Equator.
    Cyl(CylindricalEqualArea<T>),
    /// Conic
    Con(ConicEqualArea<T>),
    /// State before the parallels are set.
    #[default]
    Uninitialized,
}
impl<T> Transform for EqualArea<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        match self {
            Self::Cyl(cyl) => cyl.transform(p),
            Self::Con(con) => con.transform(p),
            Self::Uninitialized => Coord {
                x: T::nan(),
                y: T::nan(),
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

impl<T> PRConic for EqualArea<T>
where
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
        let r0 = c.sqrt() / n;
        let two = T::from(2_f64).unwrap();
        Self::Con(ConicEqualArea::new(c, n, r0, two))
    }
}

impl<T> RawBase for EqualArea<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type Builder<DRAIN: Clone> = Builder<BuilderAntimeridianResampleNoClip<DRAIN, Self, T>, T>;

    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        let mut b = Builder::new(Self::default());
        b.scale_set::<ClipAntimeridianC<ResampleNoneNoPCNC<DRAIN, Self, T>, T>>(
            T::from(155.424).unwrap(),
        );
        b.center_set::<ClipAntimeridianC<ResampleNoneNoPCNC<DRAIN, Self, T>, T>>(&Coord {
            x: T::zero(),
            y: T::from(33.6442).unwrap(),
        });
        b
    }
}
