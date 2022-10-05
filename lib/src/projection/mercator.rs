use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::projection::ScaleSet;
use crate::stream::Stream;
use crate::Transform;

use super::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use super::builder_mercator::Builder as MercatorBuilder;
use super::ProjectionRawBase;
use super::TransformExtent;

/// Projection definition.
#[derive(Clone, Copy, Debug)]
pub struct Mercator<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    two: T,
}

impl<DRAIN, T> Default for Mercator<DRAIN, T>
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

impl<DRAIN, T> ProjectionRawBase for Mercator<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    type Builder = BuilderMercatorAntimeridianResampleClip<DRAIN, Mercator<DRAIN, T>, T>;

    #[inline]
    fn builder() -> Self::Builder {
        let mut default: BuilderMercatorAntimeridianResampleClip<DRAIN, Mercator<DRAIN, T>, T> =
            MercatorBuilder::new(Mercator::default());
        // let mut default = default.reclip_convert();
        let default = default.scale_set(T::from(961_f64 / f64::TAU()).unwrap());
        default.to_owned()
    }
}

impl<DRAIN, T> TransformExtent for Mercator<DRAIN, T>
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
                x: T::max(t.x - k, x0),
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
