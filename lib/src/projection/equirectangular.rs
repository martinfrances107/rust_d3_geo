use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_factory_antimeridian;
use crate::clip::antimeridian::line::Line;
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
pub struct EquirectangularRaw<DRAIN, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    T: CoordFloat,
{
    // p_ep: PhantomData<EP>,
    p_drain: PhantomData<DRAIN>,
}

impl<DRAIN, T> Default for EquirectangularRaw<DRAIN, T>
where
    // EP: Clone + Debug + Stream<EP = EP, T = T>,
    DRAIN: Stream<EP = DRAIN, T = T>,
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
    DRAIN: Stream<EP = DRAIN, T = T>,
    // EP: Clone + Debug + Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Builder = Builder<DRAIN, Line<T>, EquirectangularRaw<DRAIN, T>, PV<T>, T>;
    type T = T;

    #[inline]
    fn builder() -> Builder<DRAIN, Line<T>, EquirectangularRaw<DRAIN, T>, PV<T>, T>
    where
        DRAIN: Stream<EP = DRAIN, T = T>,
    {
        Builder::new(
            gen_clip_factory_antimeridian(),
            EquirectangularRaw::default(),
        )
        .scale(T::from(152.63_f64).unwrap())
    }
}

impl<DRAIN, T> EquirectangularRaw<DRAIN, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    T: CoordFloat + FloatConst,
{
}

impl<DRAIN, T> Transform for EquirectangularRaw<DRAIN, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
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
