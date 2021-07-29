use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::stream::Stream;
use crate::Transform;

use super::projection::Projection;
use super::scale::Scale;

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

impl<T> Mecator<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn gen_projection_mutator<'a, DRAIN>() -> Projection<'a, DRAIN, Mecator<T>, T>
    where
        DRAIN: 'a + Default + Stream<SC = Coordinate<T>>,
    {
        let tau = T::from(2).unwrap() * T::PI();
        Projection::new(Mecator::default(), None).scale(T::from(961).unwrap() / tau)
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Transform for Mecator<T> {
    type C = Coordinate<T>;
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
