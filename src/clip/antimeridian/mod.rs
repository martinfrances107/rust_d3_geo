mod intersect;
mod line;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::stream::Stream;
use crate::stream::StreamClone;

use super::buffer::ClipBuffer;
use super::clip_base::ClipBase;
use super::BufferInTrait;

use line::Line;
// using ClipBase as a starting point.
pub struct ClipAntimeridian<T: CoordFloat + FloatConst> {
    // line: Line<T>,
    base: ClipBase<T>,
}
impl<T> ClipAntimeridian<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    pub fn new() -> Self {
        // Use the antimeridian Line version.
        let line_node = Box::new(Line::default());

        let ring_buffer_node = ClipBuffer::default();
        let mut ring_sink_node = Line::default();
        ring_sink_node.buffer_in(ring_buffer_node);

        // ring_sink.stream(ring_buffer_node);

        let base = ClipBase {
            line_node,
            ring_sink_node: Box::new(ring_sink_node),
            ring_buffer: ring_buffer_node,

            start: Coordinate {
                x: -T::PI(),
                y: -T::FRAC_PI_2(),
            },
            ..ClipBase::default()
        };
        Self { base }
    }
}

impl<T> StreamClone for ClipAntimeridian<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(*self.clone())
    }
}
impl<T> Stream for ClipAntimeridian<T> where T: CoordFloat + FloatConst + 'static {}
// impl<T> StreamInTrait<T> for ClipAntimeridian<T>
// where
//     T: CoordFloat + FloatConst,
// {
//     fn stream_in(&mut self, stream: StreamSimpleNode<T>) {
//         self.base.sink = stream;
//         let mut line = self.line_node.borrow_mut();
//         line.stream_in(stream);
//     }
// }
// impl<T> BufferInTrait<T> for ClipAntimeridian<T>
// where
//     T: CoordFloat + FloatConst,
// {
//     fn buffer_in(&mut self, stream: StreamNode<T>) {
//         self.base.sink = stream;
//         let mut line = self.line_node.borrow_mut();
//         line.stream_in(stream);
//     }
// }

// impl<T> StreamClipTrait<T> for ClipAntimeridian<T>
// where
//     T: CoordFloat + FloatConst,
// {
//     fn interpolate(
//         &self,
//         from: Option<Coordinate<T>>,
//         to: Option<Coordinate<T>>,
//         direction: T,
//         stream: StreamSimpleNode<T>,
//     ) {
//         let phi: T;
//         let mut s = stream.borrow_mut();
//         match from {
//             None => {
//                 phi = direction * T::FRAC_PI_2();
//                 s.point(
//                     Coordinate {
//                         x: -T::PI(),
//                         y: phi,
//                     },
//                     None,
//                 );
//                 s.point(
//                     Coordinate {
//                         x: T::zero(),
//                         y: phi,
//                     },
//                     None,
//                 );
//                 s.point(Coordinate { x: T::PI(), y: phi }, None);
//                 s.point(
//                     Coordinate {
//                         x: T::PI(),
//                         y: T::zero(),
//                     },
//                     None,
//                 );
//                 s.point(
//                     Coordinate {
//                         x: T::PI(),
//                         y: -phi,
//                     },
//                     None,
//                 );
//                 s.point(
//                     Coordinate {
//                         x: T::zero(),
//                         y: -phi,
//                     },
//                     None,
//                 );
//                 s.point(
//                     Coordinate {
//                         x: -T::PI(),
//                         y: -phi,
//                     },
//                     None,
//                 );
//                 s.point(
//                     Coordinate {
//                         x: -T::PI(),
//                         y: T::zero(),
//                     },
//                     None,
//                 );
//                 s.point(
//                     Coordinate {
//                         x: -T::PI(),
//                         y: phi,
//                     },
//                     None,
//                 );
//             }
//             Some(from) => {
//                 // TODO investigate is to and Option<f64>
//                 let mut s = stream.borrow_mut();
//                 let to = to.unwrap();
//                 if (from.x - to.x).abs() > T::epsilon() {
//                     let lambda = if from.x < to.x { T::PI() } else { -T::PI() };

//                     phi = direction * lambda / T::from(2).unwrap();
//                     s.point(Coordinate { x: -lambda, y: phi }, None);
//                     s.point(
//                         Coordinate {
//                             x: T::zero(),
//                             y: phi,
//                         },
//                         None,
//                     );
//                     s.point(Coordinate { x: lambda, y: phi }, None);
//                 } else {
//                     s.point(Coordinate { x: to.x, y: to.y }, None);
//                 }
//             }
//         }
//     }

//     fn point_visible(&self, p: Coordinate<T>, _z: Option<u8>) -> bool {
//         true
//     }
// }
