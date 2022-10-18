use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::ScaleSet;
use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::Builder;
use super::ClipAngleSet;
use super::ProjectionRawBase;

/// Projection definition.
#[derive(Clone, Default, Debug)]
pub struct Gnomic<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> ProjectionRawBase for Gnomic<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    type Builder = BuilderCircleResampleNoClip<DRAIN, Gnomic<DRAIN, T>, T>;

    #[inline]
    fn builder() -> Self::Builder
    where
        DRAIN: Default + Stream<EP = DRAIN, T = T>,
    {
        let mut b = Builder::new(Gnomic::default());
        b.scale_set(T::from(144.049_f64).unwrap());
        b.clip_angle_set(T::from(60_f64).unwrap())
    }
}

impl<DRAIN, T> Transform for Gnomic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let (sx, cs) = p.x.sin_cos();
        let (sy, cy) = p.y.sin_cos();
        let k = cs * cy;
        Coordinate {
            x: cy * sx / k,
            y: sy / k,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_invert(p, T::atan)
    }
}
