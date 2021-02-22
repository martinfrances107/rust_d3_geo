mod intersect;
mod line;

use num_traits::FloatConst;
use std::borrow::BorrowMut;

use geo::{CoordFloat, Coordinate};

use crate::stream::Stream;

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
        let line_node = Line::gen_node();

        let ring_buffer_node = ClipBuffer::gen_node();
        let mut ring_sink_node = Line::gen_node();

        let rs = ring_sink_node.borrow_mut();
        rs.buffer_in(ring_buffer_node.clone());

        // ring_sink.stream(ring_buffer_node);

        let base = ClipBase {
            line_node,
            ring_sink_node,
            ring_buffer_node: ring_buffer_node.clone(),
            start: Coordinate {
                x: -T::PI(),
                y: -T::FRAC_PI_2(),
            },
            ..ClipBase::default()
        };
        Self { base }
    }
}

impl<T> Stream for ClipAntimeridian<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}
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
