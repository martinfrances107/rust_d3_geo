use crate::stream::Stream;
use geo::CoordFloat;
use num_traits::FloatConst;

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
