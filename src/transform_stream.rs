use crate::stream::Stream;
use geo::CoordFloat;
use num_traits::FloatConst;

// Define the default implementation of the trait.
// use crate::stream::GeoStream;

pub type StreamProcessor<T> = Box<dyn Fn(Box<dyn Stream<T>>) -> Box<dyn Stream<T>>>;
pub struct StreamProcessorIdentity {}

impl StreamProcessorIdentity {
    pub fn new<T: CoordFloat>() -> StreamProcessor<T> {
        return Box::new(move |stream: Box<dyn Stream<T>>| {
            return stream;
        });
    }
}

impl<T> Stream<T> for StreamProcessorIdentity where T: CoordFloat + FloatConst {}

#[derive(Default)]
pub struct StreamIdentity {}

impl<T> Stream<T> for StreamIdentity where T: CoordFloat + FloatConst {}

// /// Define the default implementation of the trait.
// pub trait Stream<T> {
//     fn point(&mut self, _x: T, _y: T, _message: Option<u8>) {}
//     fn sphere(&mut self) {}
//     fn line_start(&mut self) {}
//     fn line_end(&mut self) {}
//     fn polygon_start(&mut self) {}
//     fn polygon_end(&mut self) {}
// }
