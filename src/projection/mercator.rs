use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::antimeridian::interpolate::generate as gen_interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::stream::Stream;
use crate::Transform;

use super::builder::Builder;
use super::scale::Scale;
use super::Raw;

/// MercatorRaw
///
/// Root transform.
/// Used to define a projection builder.
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
    DRAIN: Stream<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type Builder = Builder<DRAIN, Line<T>, Mercator<DRAIN, T>, PV<T>, T>;
    type T = T;

    fn builder() -> Self::Builder {
        let tau = T::from(2).unwrap() * T::PI();
        Builder::new(
            StreamNodeClipFactory::new(gen_interpolate(), Line::default(), PV::default()),
            Mercator::default(),
        )
        .scale(T::from(961).unwrap() / tau)
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
