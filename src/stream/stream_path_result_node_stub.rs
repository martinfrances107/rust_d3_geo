use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::PathResult;
use super::PathResultEnum;
use super::Stream;
use super::StreamClone;
use super::StreamInTrait;
use super::StreamPathResult;

#[derive(Clone, Copy, Debug, Default)]
pub struct StreamPathResultNodeStub<T>
where
    T: CoordFloat,
{
    /// Why the Phantom Data is required here...
    ///
    /// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
    /// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
    phantom: PhantomData<T>,
}
impl<T> PathResult for StreamPathResultNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type Out = Option<PathResultEnum<T>>;
    fn result(&mut self) -> Self::Out {
        None
    }
}
impl<T> StreamPathResult for StreamPathResultNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    fn box_clone(&self) -> Box<dyn StreamPathResult<C = Self::C, Out = Self::Out>> {
        Box::new(self.clone())
    }
}

impl<T> StreamClone for StreamPathResultNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn StreamPathResult<Out = PathResultEnum<T>, C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        panic!("Calling blox_clone on a stub.")
    }
}

impl<T> Stream for StreamPathResultNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type C = Coordinate<T>;
}

impl<T> StreamInTrait<T> for StreamPathResultNodeStub<T> where T: CoordFloat + FloatConst {}
