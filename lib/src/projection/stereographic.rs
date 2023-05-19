use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::float::FloatConst;

use crate::clip::circle::ClipCircleC;
use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::ScaleSet;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::template::ResampleNoPCNC;
use super::builder::Builder;
use super::BuilderTrait;
use super::ClipAngleSet;
use super::RawBase;

/// Projection definition.
///
/// Why is the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug, Default)]
pub struct Stereographic<T> {
    p_t: PhantomData<T>,
}

impl<T> RawBase for Stereographic<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type Builder<DRAIN: Clone> = BuilderCircleResampleNoClip<DRAIN, Self, T>;

    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        let mut b = Builder::new(Self::default());
        b.scale_set::<ClipCircleC<ResampleNoPCNC<DRAIN, Self, T>, T>>(T::from(250_f64).unwrap());
        b.clip_angle_set(T::from(142_f64).unwrap())
    }
}

impl<T> Stereographic<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn z(z: T) -> T
    where
        T: CoordFloat + FloatConst,
    {
        // Find a way to optimize this ... need a static of type T with value 2.
        T::from(2).unwrap() * z.atan()
    }
}

impl<T> Transform for Stereographic<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let (sx, cx) = p.x.sin_cos();
        let (sy, cy) = p.y.sin_cos();
        let k = T::one() + cx * cy;
        Coord {
            x: cy * sx / k,
            y: sy / k,
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        azimuthal_invert(p, Self::z)
    }
}
