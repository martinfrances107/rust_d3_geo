use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoClip;
use crate::projection::builder_mercator::ReclipConvert;
use crate::projection::Scale;
use crate::stream::Stream;
use crate::Transform;

use super::builder_mercator::Builder as MercatorBuilder;
use super::ProjectionRawBase;
use super::TransformExtent;

/// Projection definition.
#[derive(Clone, Copy, Default, Debug)]
pub struct Mercator<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> ProjectionRawBase for Mercator<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + Default + FloatConst,
{
    type Builder = BuilderMercatorAntimeridianResampleClip<DRAIN, Mercator<DRAIN, T>, T>;

    #[inline]
    fn builder() -> Self::Builder {
        let default: BuilderMercatorAntimeridianResampleNoClip<DRAIN, Mercator<DRAIN, T>, T> =
            MercatorBuilder::new(Mercator::default());
        default
            .reclip_convert()
            .scale(T::from(961_f64 / f64::TAU()).unwrap())
    }
}

impl<DRAIN, T> TransformExtent for Mercator<DRAIN, T>
where
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    type T = T;
    #[inline]
    fn transform_extent(
        self,
        k: T,
        t: Coordinate<T>,
        _x0: T,
        y0: T,
        x1: T,
        y1: T,
    ) -> [Coordinate<T>; 2] {
        [
            Coordinate {
                x: T::max(t.x - k, t.y - k),
                y: y0,
            },
            Coordinate {
                x: T::min(t.x + k, x1),
                y: y1,
            },
        ]
    }
}

impl<DRAIN, T> Transform for Mercator<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let two = T::from(2).unwrap();
        // Divergence between f64 and f32
        // when p.y  = 1.5707963267948966  (PI/2)
        // f64 outputs -37.33185619326892 which is consistent
        // with JS.
        // The f32 is different from JS. Technically
        // tan(PI/2) is NAN. and so log(NAN) is NAN.
        // The value returned
        // from tan(PI_f64/2_f64) happens to be the same
        // large number in both the JS and RUST.
        Coordinate {
            x: p.x,
            y: ((T::FRAC_PI_2() + p.y) / two).tan().ln(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let two = T::from(2).unwrap();
        Coordinate {
            x: p.x,
            y: two * (p.y.exp()).atan() - T::FRAC_PI_2(),
        }
    }
}
