use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::stream::Stream;
use crate::Transform;

use super::builder::Builder;
use super::scale::Scale;
use super::Raw;

#[derive(Clone, Copy, Debug)]
pub struct EquirectangularRaw<T>
where
    T: CoordFloat,
{
    lambda: T,
    phi: T,
}

impl<T> Default for EquirectangularRaw<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            lambda: T::zero(),
            phi: T::zero(),
        }
    }
}

impl<T> Raw for EquirectangularRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;
}

use crate::clip::antimeridian::interpolate::generate as generate_interpolate;
impl<T> EquirectangularRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    #[inline]
    pub fn gen_projection_mutator<'a, DRAIN>(
    ) -> Builder<DRAIN, Line<T>, EquirectangularRaw<T>, PV<T>, T>
    where
        DRAIN: Stream<SC = Coordinate<T>>,
    {
        Builder::new(
            StreamNodeClipFactory::new(generate_interpolate(), Line::default(), PV::default()),
            EquirectangularRaw::default(),
        )
        .scale(T::from(152.63_f64).unwrap())
    }
}

impl<T> Transform for EquirectangularRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Self::C {
        *p
    }
    fn invert(&self, p: &Coordinate<T>) -> Self::C {
        *p
    }
}
