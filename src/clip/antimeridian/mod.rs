mod intersect;
pub mod line;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::stream::CompareIntersection;
use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::stream::{Clean, CleanEnum};

// use super::buffer::ClipBuffer;
use super::clip_base::ClipBase;
// use super::BufferInTrait;
use super::clip::Clip;
use super::ClipTraitRaw;
// use super::LineEnum;
use crate::clip::ClipRaw;
// use line::Line;

#[derive(Clone, Default)]
pub struct ClipAntimeridian<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    pub base: ClipBase<T>,
}

impl<T> ClipAntimeridian<T>
where
    T: CoordFloat + FloatConst + Default,
{
    #[inline]
    pub fn gen_clip() -> Clip<T> {
        let start = Coordinate {
            x: -T::PI(),
            y: -T::PI() / T::from(2u8).unwrap(),
        };
        Clip::new(ClipRaw::Antimeridian(ClipAntimeridian::default()), start)
    }
}

// impl<T> ClipAntimeridian<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     pub fn new() -> Self {
//         // Use the antimeridian Line version.
//         // let line_node = Box::new(Line::default());

//         let ring_buffer = ClipBuffer::default();

//         let mut line = Line::default();
//         line.buffer_in(&clip_buffer);

//         // ring_sink.stream(ring_buffer_node);

//         Self {
//             base: ClipBase {
//                 line: LineEnum::Antimeridian(line),
//                 // ring_sink_node: Box::new(ring_sink_node),
//                 ring_buffer,
//                 start: Coordinate {
//                     x: -T::PI(),
//                     y: -T::FRAC_PI_2(),
//                 },
//                 ..ClipBase::default()
//             },
//         }
//     }
// }

impl<T> StreamClone for ClipAntimeridian<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        // Box::new(*self.clone())
        panic!("must fix.")
    }
}

impl<T> Clean for ClipAntimeridian<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    /// A clip trait.
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    fn clean(&self) -> CleanEnum {
        panic!("must relate code to enum")
    }
}

impl<T> ClipTraitRaw<T> for ClipAntimeridian<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type SctC = Coordinate<T>;
    type SctOC = Option<Coordinate<T>>;
    type SctT = T;
    // type SctStream = Box<dyn Stream<C = Coordinate<T>>>;
    type SctCi = CompareIntersection<T>;

    #[inline]
    fn point_visible(&self, _p: Coordinate<T>, _z: Option<u8>) -> bool {
        true
    }

    fn interpolate(
        &self,
        from: Option<Coordinate<T>>,
        to: Option<Coordinate<T>>,
        direction: T,
        stream: impl Stream<C = Coordinate<T>>,
    ) {
        let phi: T;
        let mut s = stream;
        match from {
            None => {
                phi = direction * T::FRAC_PI_2();
                s.point(
                    Coordinate {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
                s.point(
                    Coordinate {
                        x: T::zero(),
                        y: phi,
                    },
                    None,
                );
                s.point(Coordinate { x: T::PI(), y: phi }, None);
                s.point(
                    Coordinate {
                        x: T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                s.point(
                    Coordinate {
                        x: T::PI(),
                        y: -phi,
                    },
                    None,
                );
                s.point(
                    Coordinate {
                        x: T::zero(),
                        y: -phi,
                    },
                    None,
                );
                s.point(
                    Coordinate {
                        x: -T::PI(),
                        y: -phi,
                    },
                    None,
                );
                s.point(
                    Coordinate {
                        x: -T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                s.point(
                    Coordinate {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
            }
            Some(from) => {
                // TODO investigate is to and Option<f64>
                // let mut s = stream.borrow_mut();
                let to = to.unwrap();
                if (from.x - to.x).abs() > T::epsilon() {
                    let lambda = if from.x < to.x { T::PI() } else { -T::PI() };

                    phi = direction * lambda / T::from(2).unwrap();
                    s.point(Coordinate { x: -lambda, y: phi }, None);
                    s.point(
                        Coordinate {
                            x: T::zero(),
                            y: phi,
                        },
                        None,
                    );
                    s.point(Coordinate { x: lambda, y: phi }, None);
                } else {
                    s.point(Coordinate { x: to.x, y: to.y }, None);
                }
            }
        }
    }
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
