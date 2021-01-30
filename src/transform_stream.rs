use crate::stream::Stream;
use crate::stream::StreamNode;
use geo::CoordFloat;
use num_traits::FloatConst;

pub type StreamProcessor<T> = Box<dyn Fn(StreamNode<T>) -> StreamNode<T>>;
#[derive(Clone, Debug)]
pub struct StreamProcessorIdentity {}

impl StreamProcessorIdentity {
    #[inline]
    pub fn new<T: CoordFloat>() -> StreamProcessor<T> {
        Box::new(move |stream: StreamNode<T>| stream)
    }
}

impl<T> Stream<T> for StreamProcessorIdentity where T: CoordFloat + FloatConst {}

#[derive(Clone, Default, Debug)]
pub struct StreamIdentity {}

impl<T> Stream<T> for StreamIdentity where T: CoordFloat + FloatConst {}
