mod intersect;
pub mod line;
mod rejoin;

// use derivative::Derivative;
// use std::collections::VecDeque;
use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

// use crate::stream::stream_in_trait::StreamIn;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;
// use crate::stream::CompareIntersection;
// use crate::clip::compare_intersections::compare_intersections;
use crate::clip::interpolate_trait::Interpolate;
// use crate::clip::line_sink_enum::LineSinkEnum;
use crate::clip::point_visible_trait::PointVisible;
use crate::clip::Clip;
// use crate::path::PathResult;

// use crate::polygon_contains::contains;
// use crate::projection::ProjectionRawTrait;
// use crate::stream::stream_dst::StreamDst;
// use crate::clip::Clean;
// use crate::clip::clean::CleanEnum;
// use crate::clip::rejoin::Rejoin;
use crate::stream::stream_in_trait::StreamCombo;
use crate::stream::stream_in_trait::StreamIn;
use crate::stream::Stream;
// use crate::Transform;
// use super::clip::Clip;
use super::clip_base::ClipBase;
// use super::clip_sink_enum::ClipSinkEnum;
use super::line_elem::LineElem;
// use crate::clip::clean::Clean;
// use super::Clip;
// use super::ClipBaseState;
use super::ClipBuffer;
use super::ClipOpsMacro;
use super::LCB;
use clip_ops_macro_derive::ClipOpsMacro;
// use crate::clip::clip_raw::ClipRaw;
// use crate::projection::projection_trait::ProjectionTrait;

// use super::rejoin::rejoin;

use line::Line;

// #[derive(Derivative)]
// #[derivative(Debug)]
#[derive(ClipOpsMacro)]
pub struct ClipAntimeridian<SINK, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    // MutStream: Stream<SC = Coordinate<T>>,
    SINK: Default + Stream<SC = Coordinate<T>>,
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    // // #[derivative(Debug = "ignore")]
    // point_fn: fn(&mut Self, p: &Coordinate<T>, m: Option<u8>),
    // // #[derivative(Debug = "ignore")]
    // line_start_fn: fn(&mut Self),
    // // #[derivative(Debug = "ignore")]
    // line_end_fn: fn(&mut Self),
    // base: Box<ClipBase<Line<SINK, T>, SINK, T>>,
    base: ClipBase<Line<SINK, T>, SINK, T>,
}

impl<SINK, T> StreamCombo for ClipAntimeridian<SINK, T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    SINK: Default + Stream<SC = Coordinate<T>>,
{
}

// impl<SINK, T> ClipBaseState for ClipAntimeridian<SINK, T>
// where
//     SINK: Default + Stream<SC = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     type CBST = T;
//     type L = Line<SINK, T>;
//     type SINK = SINK;

//     fn get_base(self) -> ClipBase<Line<SINK, T>, SINK, T> {
//         self.base
//     }
//     fn set_polygon_started(&mut self, _started: bool) {}
//     // fn set_point_fn(&mut self, f: fn(&mut Self, p: &Coordinate<Self::CBOT>, m: Option<u8>)) {}
//     // fn set_line_start_fn(f: fn(&mut Self)) {}
//     // fn set_line_end_fn(f: fn(&mut Self)) {}
//     #[inline]
//     fn polygon_clear(&mut self) {
//         self.base.polygon.clear();
//     }

//     #[inline]
//     fn polygon_push(&mut self, v: Vec<LineElem<Self::CBST>>) {
//         self.base.polygon.push(v)
//     }

//     #[inline]
//     fn ring_clear(&mut self) {
//         self.base.ring.clear();
//     }
//     #[inline]
//     fn ring_push(&mut self, le: LineElem<Self::CBST>) {
//         self.base.ring.push(le);
//     }

//     #[inline]
//     fn ring_pop(&mut self) -> Option<LineElem<Self::CBST>> {
//         self.base.ring.pop()
//     }

//     #[inline]
//     fn ring_sink_clean(&mut self) -> CleanEnum {
//         // self.line.clean();
//         // TODO what to do here!
//         self.base.line.clean()
//     }

//     #[inline]
//     fn ring_reset(&mut self) {
//         self.base.ring = Vec::new();
//     }

//     #[inline]
//     fn set_use_point_line(&mut self, u: bool) {
//         self.base.use_point_line = u;
//     }

//     #[inline]
//     fn set_use_ring_start(&mut self, u: bool) {
//         self.base.use_ring_start = u;
//     }

//     #[inline]
//     fn set_use_ring_end(&mut self, u: bool) {
//         self.base.use_ring_end = u;
//     }

//     #[inline]
//     fn segments_clear(&mut self) {
//         self.base.segments.clear();
//     }
// }

impl<SINK, T> Clip for ClipAntimeridian<SINK, T>
where
    //     // MutStream: Stream<SC = Coordinate<T>>,
    SINK: 'static + Default + Stream<SC = Coordinate<T>>,
    //     // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
}

impl<SINK, T> ClipAntimeridian<SINK, T>
where
    SINK: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    #[inline]
    // pub fn new<PR>(projection_raw: PR) -> ClipAntimeridian<'a, SINK, T>
    pub fn new() -> ClipAntimeridian<SINK, T>
// where
    //     PR: Transform<C = Coordinate<T>>,
    {
        let start = LineElem {
            p: Coordinate {
                x: -T::PI(),
                y: -T::PI() / T::from(2u8).unwrap(),
            },
            m: None,
        };
        let line = Line::default();
        let ring_buffer = Rc::new(RefCell::new(ClipBuffer::default()));
        let mut ring_sink: Box<Line<ClipBuffer<T>, T>> = Box::new(Line::default());
        ring_sink.link_to_stream(ring_buffer.clone());

        ClipAntimeridian::<SINK, T> {
            // line_end_fn: Self::line_end_default,
            // point_fn: Self::point_default,
            // line_start_fn: Self::line_start_default,
            base: ClipBase::new(
                // projection_raw,
                line,
                ring_buffer,
                ring_sink,
                start,
            ),
        }
    }

    //     #[inline]
    //     pub fn gen_clip(projection_raw: PR) -> Clip<PR, L, T> {
    //         let start = LineElem {
    //             p: Coordinate {
    //                 x: -T::PI(),
    //                 y: -T::PI() / T::from(2u8).unwrap(),
    //             },
    //             m: None,
    //         };
    //         Self::new(
    //             projection_raw,
    //             // ClipRaw::Antimeridian(ClipAntimeridian::new(projection_raw)),
    //             start,
    //         )
    //     }
}

impl<SINK, T> PointVisible for ClipAntimeridian<SINK, T>
where
    SINK: Default + Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type PVC = Coordinate<T>;

    #[inline]
    fn point_visible(&self, _p: &Coordinate<T>, _z: Option<u8>) -> bool {
        true
    }
}

impl<SINK, T> Interpolate for ClipAntimeridian<SINK, T>
where
    SINK: Default + Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type IC = Coordinate<Self::IT>;
    type IT = T;

    // fn get_sink(&mut self) -> &mut SINK {
    //     &mut self.base.sink
    // }

    fn interpolate(
        &mut self,
        to: Option<Coordinate<T>>,
        from: Option<Coordinate<T>>,
        direction: T,
        // stream: &mut Self::IStream,
    ) {
        // let stream = Interpolate::get_sink(self);
        let mut stream = self.base.sink.borrow_mut();
        let phi: T;
        match from {
            None => {
                phi = direction * T::FRAC_PI_2();
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: T::zero(),
                        y: phi,
                    },
                    None,
                );
                stream.point(&Coordinate { x: T::PI(), y: phi }, None);
                stream.point(
                    &Coordinate {
                        x: T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: T::PI(),
                        y: -phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: T::zero(),
                        y: -phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: -phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
            }
            Some(from) => {
                let to = to.unwrap();
                if (from.x - to.x).abs() > T::epsilon() {
                    let lambda = if from.x < to.x { T::PI() } else { -T::PI() };

                    phi = direction * lambda / T::from(2).unwrap();
                    stream.point(&Coordinate { x: -lambda, y: phi }, None);
                    stream.point(
                        &Coordinate {
                            x: T::zero(),
                            y: phi,
                        },
                        None,
                    );
                    stream.point(&Coordinate { x: lambda, y: phi }, None);
                } else {
                    stream.point(&to, None);
                }
            }
        }
    }
}

/// Warning this breaks DRY, the stream is common to both ClipAntimeridian and
/// ClipCircle!!!!
impl<SINK, T> StreamIn for ClipAntimeridian<SINK, T>
where
    SINK: Default + Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SInput = SINK;
    #[inline]
    fn stream_in(&mut self, stream: SINK)
    where
        T: CoordFloat + FloatConst,
    {
        let stream = Rc::new(RefCell::new(stream));
        self.base.line.link_to_stream(stream);
    }
}

// impl<'a, SINK, T> ClipAntimeridian<'a, SINK, T>
// where
//     // Rc<PR>: Transform<C = Coordinate<T>>,
//     // PR: Transform<C = Coordinate<T>>,
//     // MutStream: Stream<SC = Coordinate<T>>,
//     SINK: Default + Stream<SC = Coordinate<T>>,
//     // STREAM: Stream<SC = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     #[inline]
//     fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_default");
//         if self.point_visible(p, None) {
//             self.base.sink.point(p, m);
//         }
//     }

//     #[inline]
//     fn point_line(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_line");
//         self.base.line.point(p, m);
//     }

//     #[inline]
//     fn line_start_default(&mut self) {
//         println!("clip line_start_default");
//         self.point_fn = Self::point_line;
//         self.line_start();
//     }

//     #[inline]
//     fn line_end_default(&mut self) {
//         println!("clip line_end_default");
//         self.point_fn = Self::point_default;
//         self.base.line.line_end();
//     }

//     #[inline]
//     fn point_ring(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_ring {:?} {:?}", p, m);
//         // println!("about to ring/push - ring_sink ");
//         // println!("self.base {:#?} ", self.base.ring_sink);
//         self.base.ring.push(LineElem { p: *p, m });
//         self.base.ring_sink.point(p, m);
//         println!("clip point_ring -- end");
//     }

//     #[inline]
//     fn ring_start(&mut self) {
//         println!("clip ring_start");
//         self.base.ring_sink.line_start();
//         self.base.ring.clear();
//         println!("end clip ring_start");
//     }

//     fn ring_end(&mut self) {
//         println!("clip ring_end  entry {:#?}", self.base.ring);
//         let le = self.base.ring[0];
//         // javascript drops m here.
//         self.point_ring(&le.p, None);
//         self.base.ring_sink.line_end();

//         let clean = self.base.ring_sink.clean();
//         let mut ring_segments = match self.base.ring_buffer.borrow_mut().result() {
//             Some(PathResultEnum::ClipBufferOutput(result)) => {
//                 // Can I find a way of doing this with the expense of dynamic conversion.
//                 result
//             }
//             Some(_) => {
//                 panic!("None buffer ");
//             }
//             None => panic!("was expecting something."),
//         };
//         println!("clip ring_end() - ring segments {:#?}", ring_segments);
//         // panic!("ring_end buffer result");
//         let n = ring_segments.len();
//         let m;
//         let mut point: Coordinate<T>;

//         self.base.ring.pop();
//         self.base.polygon.push(self.base.ring.clone());
//         // in this javascript version this value is set to NULL
//         // is my assumption that this is valid true?
//         // self.ring = None;
//         self.base.ring = Vec::new();

//         if n == 0 {
//             return;
//         }
//         println!("no intersections n, c {:?} {:?}", n, clean);
//         // No intersections.
//         match clean {
//             CleanEnum::NoIntersections => {
//                 println!("about to clean good path");
//                 // panic!("on the good path");
//                 let segment = ring_segments
//                     .pop_front()
//                     .expect("We have previously checked that the .len() is >0 ( n ) ");
//                 m = segment.len() - 1;
//                 if m > 0 {
//                     if !self.base.polygon_started {
//                         self.base.sink.polygon_start();
//                         self.base.polygon_started = true;
//                     }
//                     self.base.sink.line_start();
//                     for i in 0..m {
//                         point = segment[i].p;
//                         self.base.sink.point(&point, None);
//                     }
//                     self.base.sink.line_end();
//                 }
//                 return;
//             }
//             CleanEnum::IntersectionsRejoin => {
//                 // Rejoin connected segments.
//                 // TODO reuse ringBuffer.rejoin()?
//                 if n > 1 {
//                     println!("ring_segments before fb {:#?}", ring_segments);
//                     let pb = [
//                         ring_segments.pop_back().unwrap(),
//                         ring_segments.pop_front().unwrap(),
//                     ]
//                     .concat();
//                     ring_segments.push_back(pb);
//                 }
//             }
//             CleanEnum::IntersectionsOrEmpty => {
//                 // No-op
//             }
//             CleanEnum::Undefined => {
//                 panic!("must be defined by now.")
//             }
//         }
//         println!("final segments before filter {:#?}", ring_segments);
//         // panic!("final segments");
//         let filtered: Vec<Vec<LineElem<T>>> = ring_segments
//             .into_iter()
//             .filter(|segment| segment.len() > 1)
//             .collect();
//         self.base.segments.push_back(filtered);
//     }
// }

// /// Warning this breaks DRY, the stream is common to both ClipAntimeridian and
// /// ClipCircle!!!!
impl<'a, SINK, T> Stream for ClipAntimeridian<SINK, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    // MutStream: Stream<SC = Coordinate<T>>,
    SINK: Default + Stream<SC = Coordinate<T>>,
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;
    // type SD = SD;
    // type ST = T;
    // fn get_dst(
    //     &self,
    // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
    // {
    //     self.base.sink.get_dst()
    // }

    #[inline]
    fn point(&mut self, _p: &Coordinate<T>, _m: Option<u8>) {
        // (self.point_fn)(self, p, m);
    }

    #[inline]
    fn line_start(&mut self) {
        // (self.line_start_fn)(self);
    }

    #[inline]
    fn line_end(&mut self) {
        // (self.line_end_fn)(self);
    }

    fn polygon_start(&mut self) {
        println!("clip  polygon start");
        // self.point_fn = Self::point_ring;
        // self.line_start_fn = Self::ring_start;
        // self.line_end_fn = Self::ring_end;
        self.base.segments.clear();
        self.base.polygon.clear();
    }
    fn polygon_end(&mut self) {
        println!("clip polygon_end");
        // self.point_fn = Self::point_default;
        // self.line_start_fn = Self::line_start_default;
        // self.line_end_fn = Self::line_end_default;
        println!("about to merge {:#?}", self.base.segments);
        let segments_merged: Vec<Vec<LineElem<T>>> =
            self.base.segments.clone().into_iter().flatten().collect();
        // let start_inside = contains(&self.base.polygon, &self.base.start);
        let start_inside = true;

        if !segments_merged.is_empty() {
            println!("mergeed is not empty {:#?}", self.base.segments);
            // panic!("pause here");
            if !self.base.polygon_started {
                // self.base.sink.polygon_start();
                self.base.polygon_started = true;
            }
            println!("into rejoin this path");

            // self.rejoin(
            //     &segments_merged,
            //     compare_intersections,
            //     start_inside,
            //     // self,
            //     // self.interpolate(),
            //     // &mut self.base.sink,
            // );
        } else if start_inside {
            if !self.base.polygon_started {
                // self.base.sink.polygon_start();
                self.base.polygon_started = true;
            }
            // self.base.sink.line_start();
            self.interpolate(None, None, T::one());
            // self.base.sink.line_end();
        };
        if self.base.polygon_started {
            // self.base.sink.polygon_end();
            self.base.polygon_started = false;
        }
        self.base.segments.clear();
        self.base.polygon.clear();
        println!("clip polygon_end -- exit");
    }

    fn sphere(&mut self) {
        // self.base.sink.polygon_start();
        // self.base.sink.line_start();
        // self.interpolate(None, None, T::one(), &mut self.base.sink);
        self.interpolate(None, None, T::one());
        // self.base.sink.line_end();
        // self.base.sink.polygon_end();
    }
}
