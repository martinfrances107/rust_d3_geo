use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_factory_antimeridian;
use crate::clip::antimeridian::pv::PV;
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
pub struct EquirectangularRaw<DRAIN, EP, T>
where
    DRAIN: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    // p_ep: PhantomData<EP>,
    p_drain: PhantomData<DRAIN>,
}

impl<DRAIN, EP, T> Default for EquirectangularRaw<DRAIN, EP, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    DRAIN: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
        }
    }
}

impl<DRAIN, EP, T> Raw<T> for EquirectangularRaw<DRAIN, EP, T>
where
    DRAIN: Stream<EP = EP, T = T>,
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Builder = Builder<DRAIN, EP, EquirectangularRaw<DRAIN, EP, T>, PV<T>, T>;
    type T = T;

    #[inline]
    fn builder() -> Builder<DRAIN, EP, EquirectangularRaw<DRAIN, EP, T>, PV<T>, T>
    where
        DRAIN: Stream<EP = EP, T = T>,
    {
        Builder::new(
            gen_clip_factory_antimeridian(),
            EquirectangularRaw::default(),
        )
        .scale(T::from(152.63_f64).unwrap())
    }
}

impl<DRAIN, EP, T> EquirectangularRaw<DRAIN, EP, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    DRAIN: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
}

impl<DRAIN, EP, T> Transform for EquirectangularRaw<DRAIN, EP, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    DRAIN: Stream<EP = EP, T = T>,
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
