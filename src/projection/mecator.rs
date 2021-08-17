use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::clip::antimeridian::interpolate::generate as gen_interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::stream::Stream;
use crate::Transform;

use super::builder::Builder;
use super::scale::Scale;
use super::Raw;

#[derive(Clone, Copy, Debug)]
pub struct Mecator<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> Default for Mecator<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Mecator {
            phantom: PhantomData::<T>,
        }
    }
}

impl<T> Raw for Mecator<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;
}
impl<T> Mecator<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn gen_projection_mutator<DRAIN>() -> Builder<DRAIN, Line<T>, Mecator<T>, PV<T>, T>
    where
        DRAIN: Stream<T = T>,
    {
        let tau = T::from(2).unwrap() * T::PI();
        Builder::new(
            StreamNodeClipFactory::new(gen_interpolate(), Line::default(), PV::default()),
            Mecator::default(),
        )
        .scale(T::from(961).unwrap() / tau)
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Transform for Mecator<T> {
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
