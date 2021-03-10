use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::stream_dummy::StreamDummy;
use super::Stream;
use super::StreamClone;
use super::StreamInTrait;

/// A Stub acts as a black hole.
/// A StreamIdentity acts as a 'pass through' node.
pub struct StreamIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    stream: Box<dyn Stream<C = Coordinate<T>>>,
}

impl<T> Default for StreamIdentity<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    fn default() -> Self {
        Self {
            stream: Box::new(StreamDummy::default()),
        }
    }
}

impl<T> StreamClone for StreamIdentity<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(StreamIdentity::<T>::default())
    }
}

impl<T> Stream for StreamIdentity<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type C = Coordinate<T>;
    #[inline]
    fn point(&mut self, p: Self::C, m: Option<u8>) {
        self.stream.point(p, m);
    }
    #[inline]
    fn sphere(&mut self) {
        self.stream.sphere();
    }
    #[inline]
    fn line_start(&mut self) {
        self.stream.line_start();
    }
    #[inline]
    fn line_end(&mut self) {
        self.stream.line_end();
    }
    #[inline]
    fn polygon_start(&mut self) {
        self.stream.polygon_start();
    }
    #[inline]
    fn polygon_end(&mut self) {
        self.stream.polygon_end();
    }
}

impl<T> StreamInTrait<T> for StreamIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_in(&mut self, stream: Box<dyn Stream<C = Coordinate<T>>>) {
        self.stream = stream;
    }
}
