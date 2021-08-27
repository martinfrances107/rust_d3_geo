use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::antimeridian::interpolate::generate as gen_interpolate;
use crate::clip::circle::line::Line;
use crate::clip::circle::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::Builder;
use super::scale::Scale;
use super::Raw;

/// GnomicRaw
///
/// Root transform.
/// Used to define a projection builder.
#[derive(Copy, Clone, Debug)]
pub struct Gnomic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Gnomic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        Gnomic {
            p_drain: PhantomData::<DRAIN>,
            p_t: PhantomData::<T>,
        }
    }
}

impl<DRAIN, T> Raw<T> for Gnomic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type Builder = Builder<DRAIN, Line<T>, Gnomic<DRAIN, T>, PV<T>, T>;
    type T = T;

    fn builder() -> Self::Builder
    where
        DRAIN: Stream<T = T>,
    {
        let g = Gnomic::default();
        Builder::new(
            StreamNodeClipFactory::new(gen_interpolate(), Line::<T>::default(), PV::<T>::default()),
            g,
        )
        .scale(T::from(144.049_f64).unwrap())
        .clip_angle(T::from(60_f64).unwrap())
    }
}

impl<DRAIN, T> Gnomic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn atan(z: T) -> T
    where
        T: CoordFloat + FloatConst,
    {
        // Find a way to optimize this ... need a static of type T with value 2.
        z.atan()
    }
}

impl<DRAIN, T> Transform for Gnomic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
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
        azimuthal_invert(p, Self::atan)
    }
}
