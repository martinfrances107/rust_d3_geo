use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::clip::antimeridian::interpolate::generate as generate_interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::stream::Stream;
use crate::Transform;

use super::builder::Builder;
use super::Raw;
use super::Scale;

/// EquirectangularRaw
///
/// Root transform.
/// Used to define a projection builder.
#[derive(Clone, Copy, Debug)]
pub struct EquirectangularRaw<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: CoordFloat,
{
    p_drain: PhantomData<DRAIN>,
}

impl<DRAIN, T> Default for EquirectangularRaw<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
        }
    }
}

impl<DRAIN, T> Raw<T> for EquirectangularRaw<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Builder = Builder<DRAIN, Line<T>, EquirectangularRaw<DRAIN, T>, PV<T>, T>;
    type T = T;

    #[inline]
    fn builder() -> Builder<DRAIN, Line<T>, EquirectangularRaw<DRAIN, T>, PV<T>, T>
    where
        DRAIN: Stream<T = T>,
    {
        Builder::new(
            StreamNodeClipFactory::new(generate_interpolate(), Line::default(), PV::default()),
            EquirectangularRaw::default(),
        )
        .scale(T::from(152.63_f64).unwrap())
    }
}

impl<DRAIN, T> EquirectangularRaw<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
}

impl<DRAIN, T> Transform for EquirectangularRaw<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: CoordFloat,
{
    type T = T;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        *p
    }
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        *p
    }
}
