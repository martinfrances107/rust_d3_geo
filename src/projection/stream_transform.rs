use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;
use crate::Transform;

use super::StreamNode;

#[derive(Clone, Debug)]
pub struct StreamTransform<T, TRANSFORMER>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    TRANSFORMER: Transform<T = T>,
{
    pub transformer: TRANSFORMER,
}

// impl<STREAM, T> Default for StreamTransform<STREAM, T>
// where
//     // P: Transform<TcC = Coordinate<T>>,
//     // PR: ProjectionRawTrait,
//     STREAM: Stream<T=T> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     fn default() -> Self {
//         Self {
//             transform: RotateRadiansEnum::I(RotationIdentity::default()),
//             stream: STREAM::default(),
//         }
//     }
// }

// impl<STREAM, T> StreamIn for StreamTransform<STREAM, T>
// where
//     STREAM: Stream<T=T> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     // type C = Coordinate<T>;
//     // type T = T;
//     // type SD = SD;
//     type SInput = STREAM;
//     #[inline]
//     fn stream_in(&mut self, stream: STREAM) {
//         self.stream = stream;
//     }
// }

impl<T, TRANSFORMER> StreamTransform<T, TRANSFORMER>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
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

// impl<T, TRANSFORMER> Transform for StreamTransform<T, TRANSFORMER>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
//     TRANSFORMER: Transform<C = Coordinate<T>>,
// {
//     type C = Coordinate<T>;
//     fn transform(&self, p: &Self::C) -> Self::C {
//         self.transform.transform(p)
//     }
//     fn invert(&self, p: &Self::C) -> Self::C {
//         self.transform.invert(p)
//     }
// }

impl<SINK, T, TRANSFORMER> Stream for StreamNode<StreamTransform<T, TRANSFORMER>, SINK, T>
where
    SINK: Stream<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    TRANSFORMER: Transform<T = T>,
{
    type T = T;

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        // Warning the javascript version return the value below but I think it break the implied spec!!!!
        self.sink
            .borrow_mut()
            .point(&self.raw.transformer.transform(&p), m);
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
