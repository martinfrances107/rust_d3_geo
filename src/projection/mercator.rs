use num_traits::AsPrimitive;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::stream::Stream;
use crate::Transform;

use super::mercator_builder::MercatorBuilder;
use super::Raw;
use super::Scale;

/// Defines a projection.
#[derive(Clone, Copy, Debug)]
pub struct Mercator<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Mercator<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        Mercator {
            p_drain: PhantomData::<DRAIN>,
            p_t: PhantomData::<T>,
        }
    }
}

impl<DRAIN, T> Raw<T> for Mercator<DRAIN, T>
where
    DRAIN: Stream<T = T> + Default,
    T: 'static + AsPrimitive<T> + CoordFloat + FloatConst,
{
    type Builder = MercatorBuilder<DRAIN, Line<T>, Mercator<DRAIN, T>, PV<T>, T>;
    type T = T;

    fn builder() -> Self::Builder {
        let tau = T::from(2).unwrap() * T::PI();
        MercatorBuilder::new(Mercator::default()).scale(T::from(961).unwrap() / tau)
    }
}

impl<DRAIN, T> Transform for Mercator<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let two = T::from(2).unwrap();
        // Divergence between f64 and f32
        // when p.y  = 1.5707963267948966
        // f64 outputs the correct result -37.33185619326892
        // The f32 value is junk... at least different from JS
        // tan(pi/2) is undefined. The value returned
        //from tan(pi_f64/2_f64) happens to be the same
        // large number in both the JS and RUST.
        dbg!(((T::FRAC_PI_2() + p.y) / two).tan().ln());
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
