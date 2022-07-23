use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::ClipAngleSet;
use crate::projection::Scale;
use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::Builder;
use super::ProjectionRawBase;

/// Projection definition.
#[derive(Clone, Default, Debug)]
pub struct Gnomic<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> ProjectionRawBase for Gnomic<DRAIN, T>
where
    DRAIN: Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst,
{
    type Builder = BuilderCircleResampleNoClip<DRAIN, Gnomic<DRAIN, T>, T>;

    fn builder() -> Self::Builder
    where
        DRAIN: Default + Stream<EP = DRAIN, T = T>,
    {
        Builder::new(Gnomic::default())
            .scale(T::from(144.049_f64).unwrap())
            .clip_angle(T::from(60_f64).unwrap())
    }
}

impl<DRAIN, T> Transform for Gnomic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let cy = p.y.cos();
        let k = p.x.cos() * cy;
        Coordinate {
            x: cy * p.x.sin() / k,
            y: p.y.sin() / k,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_invert(p, T::atan)
    }
}
