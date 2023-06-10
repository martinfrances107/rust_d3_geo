use core::fmt::Debug;
use core::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::float::FloatConst;

use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::ScaleSet;
use crate::Transform;

use super::azimuthal::azimuthal_invert;

use super::builder::Builder;
use super::BuilderTrait;
use super::ClipAngleSet;
use super::RawBase;

/// Projection definition. ``Gnomic::builder()`` returns a builder.
#[derive(Clone, Default, Debug)]
pub struct Gnomic<T> {
    p_t: PhantomData<T>,
}

impl<T> RawBase for Gnomic<T>
where
    T: 'static + CoordFloat + Default + FloatConst,
{
    type Builder<DRAIN: Clone> = BuilderCircleResampleNoClip<DRAIN, Self, T>;

    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        let mut b = Builder::new(Self::default());
        b.scale_set(T::from(144.049_f64).unwrap());
        b.clip_angle_set(T::from(60_f64).unwrap())
    }
}

impl<T> Transform for Gnomic<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let (sx, cx) = p.x.sin_cos();
        let (sy, cy) = p.y.sin_cos();
        let k = cx * cy;
        Coord {
            x: cy * sx / k,
            y: sy / k,
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        azimuthal_invert(p, T::atan)
    }
}
