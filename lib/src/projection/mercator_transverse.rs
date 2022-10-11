use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::stream::Stream;
use crate::Transform;

use super::builder_mercator_transverse::types::BuilderMercatorTransverseAntimeridianResampleClip;
use super::builder_mercator_transverse::Builder as MercatorTraverseBuilder;
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
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst,
{
    type Builder =
        BuilderMercatorTransverseAntimeridianResampleClip<DRAIN, MercatorTransverse<DRAIN, T>, T>;

    #[inline]
    fn builder() -> Self::Builder {
        MercatorTraverseBuilder::new(MercatorTransverse::default())
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
            x: ((T::FRAC_PI_2() + p.y) / self.two).tan().ln(),
            y: -p.x,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        Coordinate {
            x: -p.y,
            y: self.two * (p.x.exp()).atan() - T::FRAC_PI_2(),
        }
    }
}

impl<DRAIN, T> TransformExtent for MercatorTransverse<DRAIN, T>
where
    T: CoordFloat,
{
    type T = T;
    #[inline]
    fn transform_extent(
        self,
        k: T,
        t: Coordinate<T>,
        x0: T,
        y0: T,
        x1: T,
        y1: T,
    ) -> [Coordinate<T>; 2] {
        [
            Coordinate {
                x: x0,
                y: T::max(t.x - k, y0),
            },
            Coordinate {
                x: x1,
                y: T::min(t.y + k, y1),
            },
        ]
    }
}
