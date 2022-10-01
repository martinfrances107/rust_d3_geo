use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::projection::builder_mercator::ReclipConvert;
use crate::projection::ScaleSet;
use crate::stream::Stream;
use crate::Transform;

use super::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use super::builder_mercator::types::BuilderMercatorAntimeridianResampleNoClip;
use super::builder_mercator::Builder as MercatorBuilder;
use super::ProjectionRawBase;
use super::TransformExtent;

/// Projection definition.
#[derive(Clone, Copy, Debug)]
pub struct MercatorTransverse<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    two: T,
}

impl<DRAIN, T> Default for MercatorTransverse<DRAIN, T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
            two: T::from(2_f64).unwrap(),
        }
    }
}

impl<DRAIN, T> ProjectionRawBase for MercatorTransverse<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    type Builder = BuilderMercatorAntimeridianResampleClip<DRAIN, MercatorTransverse<DRAIN, T>, T>;

    #[inline]
    fn builder() -> Self::Builder {
        let default: BuilderMercatorAntimeridianResampleNoClip<
            DRAIN,
            MercatorTransverse<DRAIN, T>,
            T,
        > = MercatorBuilder::new(MercatorTransverse::default());
        // default.scale_set(T::from(961_f64 / f64::TAU()).unwrap())
        todo!();
    }
}

impl<DRAIN, T> Transform for MercatorTransverse<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        Coordinate {
            x: p.x,
            y: ((T::FRAC_PI_2() + p.y) / self.two).tan().ln(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        Coordinate {
            x: p.x,
            y: self.two * (p.y.exp()).atan() - T::FRAC_PI_2(),
        }
    }
}
