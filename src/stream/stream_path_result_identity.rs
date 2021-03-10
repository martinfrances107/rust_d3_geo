use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::path::PathResult;
use crate::path::PathResultEnum;

use super::Stream;
use super::StreamClone;
use super::StreamInTrait;
use super::StreamPathResult;

#[derive(Clone, Copy, Default, Debug)]
pub struct StreamPathResultIdentity<T>
where
    T: CoordFloat,
{
    /// Why the Phantom Data is required here...
    ///
    /// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
    /// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
    phantom: PhantomData<T>,
}

impl<T> StreamPathResult for StreamPathResultIdentity<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    fn box_clone(&self) -> Box<dyn StreamPathResult<C = Self::C, Out = Self::Out>> {
        Box::new(self.clone())
    }
}

impl<T> StreamClone for StreamPathResultIdentity<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(self.clone())
    }
}
impl<T> Stream for StreamPathResultIdentity<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type C = Coordinate<T>;
}
impl<T> StreamInTrait<T> for StreamPathResultIdentity<T> where T: CoordFloat + FloatConst {}
impl<T> PathResult for StreamPathResultIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    type Out = Option<PathResultEnum<T>>;
    fn result(&mut self) -> Self::Out {
        None
    }
}
