use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_factory_antimeridian;
use crate::clip::circle::line::Line;
use crate::clip::circle::pv::PV;

use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::Builder;
use super::ClipAngle;
use super::Raw;
use super::Scale;

/// GnomicRaw
///
/// Root transform.
/// Used to define a projection builder.
#[derive(Clone, Debug)]
pub struct Gnomic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    p_drain: PhantomData<DRAIN>,
    // p_ep: PhantomData<EP>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Gnomic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        Gnomic {
            p_drain: PhantomData::<DRAIN>,
            // p_ep: PhantomData::<EP>,
            p_t: PhantomData::<T>,
        }
    }
}

impl<DRAIN, T> Raw<T> for Gnomic<DRAIN, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    // EP: Clone + Debug + Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Builder = Builder<DRAIN, Line<T>, Gnomic<DRAIN, T>, PV<T>, T>;
    type T = T;

    fn builder() -> Self::Builder
    where
        DRAIN: Stream<EP = DRAIN, T = T>,
    {
        Builder::new(gen_clip_factory_antimeridian(), Gnomic::default())
            .scale(T::from(144.049_f64).unwrap())
            .clip_angle(T::from(60_f64).unwrap())
    }
}

impl<DRAIN, EP, T> Transform for Gnomic<DRAIN, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    DRAIN: Stream<EP = EP, T = T>,
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
        azimuthal_invert(p, T::atan)
    }
}
