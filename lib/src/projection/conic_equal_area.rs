use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::math::EPSILON;
use crate::projection::ScaleSet;
use crate::stream::Stream;
use crate::Transform;

use super::builder::types::BuilderAntimeridianResampleNoClip;
use super::builder::Builder;
use super::cylindrical_equal_area::CylindricalEqualArea;
use super::CenterSet;
use super::ProjectionRawBase;

///Projection definition.
///
/// Why is the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Debug)]
pub struct ConicEqualArea<DRAIN, T> {
    c: T,
    p_drain: PhantomData<DRAIN>,
    n: T,
    r0: T,
    two: T,
}

/// ConicEqualAreaRaw return type.
///
/// Depending constgruction parameters
/// one of two Projection types are returned.
#[derive(Clone, Debug)]
pub enum EqualArea<DRAIN, T> {
    /// Parallels symetical around the Equator.
    Cyl(CylindricalEqualArea<DRAIN, T>),
    /// Conic
    Con(ConicEqualArea<DRAIN, T>),
}
impl<DRAIN, T> Transform for EqualArea<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            EqualArea::Cyl(cyl) => cyl.transform(p),
            EqualArea::Con(con) => con.transform(p),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            EqualArea::Cyl(cyl) => cyl.invert(p),
            EqualArea::Con(con) => con.invert(p),
        }
    }
}

impl<DRAIN, T> ConicEqualArea<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    pub(super) fn generate(y0: T, y1: T) -> EqualArea<DRAIN, T> {
        let two = T::from(2_f64).unwrap();
        let sy0 = y0.sin();
        let n = (sy0 + y1.sin()) / two;

        // Are the parallels symmetrical around the Equator?
        if n.abs() < T::from(EPSILON).unwrap() {
            return EqualArea::Cyl(CylindricalEqualArea::new(y0));
        }
        let c = T::one() + sy0 * (two * n - sy0);
        EqualArea::Con(ConicEqualArea {
            p_drain: PhantomData::<DRAIN>,
            c: T::one() + sy0,
            r0: c.sqrt() / n,
            n,
            two,
        })
    }
    #[inline]
    /// Phi0 value in radians.
    pub fn builder_with_phi0_phi1(
        y0: T,
        y1: T,
    ) -> BuilderAntimeridianResampleNoClip<DRAIN, EqualArea<DRAIN, T>, T> {
        let mut b = Builder::new(ConicEqualArea::generate(y0, y1));
        b.scale_set(T::from(155.424).unwrap())
            .center_set(&Coordinate {
                x: T::zero(),
                y: T::from(33.6442).unwrap(),
            });
        b
    }
}

impl<DRAIN, T> ProjectionRawBase for ConicEqualArea<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    type Builder = BuilderAntimeridianResampleNoClip<DRAIN, EqualArea<DRAIN, T>, T>;
    #[inline]
    fn builder() -> Self::Builder {
        Self::builder_with_phi0_phi1(T::zero(), T::FRAC_PI_3())
    }
}

impl<DRAIN, T> Transform for ConicEqualArea<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let r = (self.c - self.two * self.n * p.y.sin()).sqrt() / self.n;
        let x = p.x * self.n;
        Coordinate {
            x: r * x.sin(),
            y: self.r0 - r * x.cos(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let r0y = self.r0 - p.y;
        let mut l = p.x.atan2(r0y.abs()) * r0y.signum();
        if r0y * self.n < T::zero() {
            l = l - T::PI() * p.x.signum() * r0y.signum();
        }

        Coordinate {
            x: l / self.n,
            y: ((self.c - (p.x * p.x + r0y * r0y) * self.n * self.n) / (self.two * self.n)).asin(),
        }
    }
}
