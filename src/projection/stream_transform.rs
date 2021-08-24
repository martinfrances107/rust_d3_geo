use geo::{CoordFloat, Coordinate};

use crate::stream::Stream;
use crate::Transform;

use super::StreamNode;

#[derive(Clone, Debug)]
pub struct StreamTransform<T, TRANSFORMER>
where
    T: CoordFloat,
    TRANSFORMER: Transform<T = T>,
{
    pub transformer: TRANSFORMER,
}

impl<T, TRANSFORMER> StreamTransform<T, TRANSFORMER>
where
    T: CoordFloat,
    TRANSFORMER: Transform<T = T>,
{
    #[inline]
    pub fn new(transformer: TRANSFORMER) -> StreamTransform<T, TRANSFORMER> {
        {
            Self {
                // stream: stream,
                transformer,
            }
        }
    }
}

impl<T, TRANSFORMER> Transform for StreamTransform<T, TRANSFORMER>
where
    T: CoordFloat,
    TRANSFORMER: Transform<T = T>,
{
    type T = T;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        dbg!("stream_transform transform() ---------------------------------");
        self.transformer.transform(p)
    }
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        self.transformer.invert(p)
    }
}

impl<SINK, T, TRANSFORMER> Stream for StreamNode<StreamTransform<T, TRANSFORMER>, SINK, T>
where
    SINK: Stream<T = T>,
    T: CoordFloat,
    TRANSFORMER: Transform<T = T>,
{
    type T = T;

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        // Warning the javascript version return the value below but I think it break the implied spec!!!!
        dbg!("stream_transform point() ---------------------------------");
        self.sink
            .borrow_mut()
            .point(&self.raw.transformer.transform(p), m);
    }

    #[inline]
    fn sphere(&mut self) {
        self.sink.borrow_mut().sphere();
    }

    #[inline]
    fn line_start(&mut self) {
        self.sink.borrow_mut().line_start();
    }

    #[inline]
    fn line_end(&mut self) {
        self.sink.borrow_mut().line_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.sink.borrow_mut().polygon_start();
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.sink.borrow_mut().polygon_end();
    }
}
