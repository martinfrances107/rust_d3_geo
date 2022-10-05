use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::stream::Stream;
use crate::Transform;

use super::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
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
        let default: BuilderMercatorAntimeridianResampleClip<
            DRAIN,
            MercatorTransverse<DRAIN, T>,
            T,
        > = MercatorBuilder::new(MercatorTransverse::default());
        default
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
