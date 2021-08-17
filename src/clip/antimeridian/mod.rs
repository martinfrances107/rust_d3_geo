pub mod interpolate;
mod intersect;
pub mod line;
pub mod pv;

use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::generate as gen_interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use crate::Transform;

pub fn gen_clip_factory_antimeridian<PR, SINK, T>(
) -> StreamNodeClipFactory<Line<T>, PR, PV<T>, SINK, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    SINK: Stream<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    StreamNodeClipFactory::new(gen_interpolate::<SINK, T>(), Line::default(), PV::default())
}

// fn gen_antimmeridian() -> Clip {
//     Clip {
//         point_visible_factory: StreamNodeFactory{Point}
//         line_factory,
//         interpolate_factory,
//         start,
//     }
// }
// impl<T> NodeFactory for StreamNodeFactory<Clip<Interpolate<T>, Line<T>, PV<T>, T>, T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     type T = T;
//     fn generate(
//         &self,
//         sink: Rc<RefCell<dyn Stream<SC = Coordinate<Self::T>>>>,
//     ) -> Rc<RefCell<dyn Stream<SC = Coordinate<Self::T>>>>
// // where
//     //     PR: Transform<C = Coordinate<T>>,
//     {
//         let start = LineElem {
//             p: Coordinate {
//                 x: -T::PI(),
//                 y: -T::PI() / T::from(2u8).unwrap(),
//             },
//             m: None,
//         };
//         let line_factory = StreamNodeFactory::new(Line::default()).generate(sink);
//         let ring_buffer = Rc::new(RefCell::new(ClipBuffer::default()));
//         let mut ring_sink = line_factory.generate(ring_buffer);

//         Rc::new(RefCell::new(Clip {
//             pv: PV {},
//             line_factory: StreamNodeFactory::new(Line::default()),
//             interpolate: StreamNodeFactory::new(Interpolate {}),
//             start,
//         }))
//     }
// }

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
//             Some(ResultEnum::ClipBufferOutput(result)) => {
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
