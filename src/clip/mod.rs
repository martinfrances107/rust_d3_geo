pub mod antimeridian;
pub mod buffer;
pub mod circle;
mod rejoin;
use crate::path::PathResult;
use crate::stream::Ci;
use crate::stream::Stream;
use crate::stream::StreamClipLineNode;
use crate::stream::StreamClipLineNodeStub;
use crate::stream::StreamClipTrait;
use crate::stream::StreamInTrait;
use crate::stream::StreamPathResult;
use crate::stream::StreamPathResultNode;
use crate::stream::{StreamPathResultNodeStub, StreamSimpleNode};
use buffer::{ClipBuffer, LineElem};
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::{cell::RefCell, rc::Rc};
// pub type InterpolateFn<T> =
// Rc<Box<dyn Fn(Option<Coordinate<T>>, Option<Coordinate<T>>, T, StreamSimpleNode<T>)>>;
// pub type PointVisibleFn<T> = Rc<Box<dyn Fn(Coordinate<T>, Option<u8>) -> bool>>;

// pub type CompareIntersectionFn<T> = Rc<Box<dyn Fn(Ci<T>, Ci<T>) -> T>>;
pub type ClipNode<T> = Rc<RefCell<Box<dyn StreamClipTrait<T>>>>;

// #[derive(Clone, Debug)]
// pub struct PreClipNodeStub {}
// pub struct PostClipNodeStub {}

// impl PreClipNodeStub {
//     #[inline]
//     pub fn gen_node<T>() -> PreClipNode<T>
//     where
//         T: CoordFloat + FloatConst,
//     {
//         Rc::new(RefCell::new(Box::new(ClipTraitIdentity {})))
//     }
// }
// impl PostClipNodeStub {
//     #[inline]
//     pub fn gen_node<T>() -> PostClipNode<T>
//     where
//         T: CoordFloat + FloatConst,
//     {
//         Rc::new(RefCell::new(Box::new(ClipTraitIdentity {})))
//     }
// }

pub trait BufferInTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn buffer_in(&mut self, sink: StreamPathResultNode<T>);
}

pub struct ClipBase<T: CoordFloat + FloatConst> {
    line_node: StreamClipLineNode<T>,
    polygon_started: bool,
    polygon: Vec<Vec<Coordinate<T>>>,
    ring: Vec<Coordinate<T>>,
    ring_buffer_node: StreamPathResultNode<T>,
    ring_sink_node: StreamClipLineNode<T>,
    segments: Vec<Vec<LineElem<T>>>,
    start: Coordinate<T>,
    use_ring: bool,
    use_ring_end: bool,
    use_ring_start: bool,
    sink: StreamPathResultNode<T>,
}

impl<T> Default for ClipBase<T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        Self {
            line_node: StreamClipLineNodeStub::new(),
            polygon_started: false,
            polygon: vec![vec![]],
            ring: vec![],
            ring_buffer_node: StreamPathResultNodeStub::new(),
            ring_sink_node: StreamClipLineNodeStub::new(),
            segments: vec![vec![]],
            use_ring: false,
            use_ring_end: false,
            use_ring_start: false,
            sink: StreamPathResultNodeStub::new(),
            start: Coordinate {
                x: -T::PI(),
                y: -T::FRAC_PI_2(),
            },
        }
    }
}

impl<T> ClipBase<T>
where
    T: CoordFloat + FloatConst,
{
    fn point_ring(&mut self, p: Coordinate<T>, _m: Option<u8>) {
        self.ring.push(p);
        let mut rs = self.ring_sink_node.borrow_mut();
        rs.point(p, None);
    }

    fn ring_start(&mut self) {
        let mut sink = self.ring_sink_node.borrow_mut();
        sink.line_start();
        self.ring = Vec::new();
    }

    fn ring_end(&mut self) {
        self.point_ring(self.ring[0], None);
        let mut ring_sink = self.ring_sink_node.borrow_mut();
        ring_sink.line_end();

        // let clean = ring_sink.clean();
        // let mut ring_buffer = self.ring_buffer_node.borrow_mut();
        // let ring_segments = match ring_buffer.result() {
        //     PathResultEnum::ClipBufferOutput(result) => {
        //         // Can I find a way of doing this with the expense of dynamic conversion.
        //         result
        //     }
        //     _ => {
        //         panic!("was expectcing a path result");
        //     }
        // };

        // let n = ring_segments.len();
        // let m;
        // // let segment: Vec<Vec<Coordinate<T>>;
        // // let point;

        // self.ring.pop();
        // self.polygon.push(self.ring);
        // // in this javascript version this value is set to NULL
        // // is my assumption that this is valid true?
        // // self.ring = None;
        // self.ring = Vec::new();

        // if n != 0 {
        //     return;
        // }

        // // No intersections.
        // match clean {
        //     CleanEnum::NoIntersections => {
        //         // let test = ring_segments.first();
        //         // let test1 = test.unwrap();
        //         // let test2 = test1.clone();
        //         let segment = ring_segments.first().unwrap().clone();
        //         m = segment.len() - 1;
        //         if m > 0 {
        //             let mut sink = self.sink.borrow_mut();
        //             if !self.polygon_started {
        //                 sink.polygon_start();
        //                 self.polygon_started = true;
        //             }
        //             sink.line_start();
        //             for i in 0..m {
        //                 let le = segment[i];
        //                 sink.point(le.p, le.m);
        //             }
        //             sink.line_end();
        //         }
        //         return;
        //     }
        //     CleanEnum::IntersectionsRejoin => {
        //         // Rejoin connected segments.
        //         // TODO reuse ringBuffer.rejoin()?
        //         if n > 1 {
        //             // ringSegments.push(ringSegments.pop().concat(ringSegments.shift()));

        //             let mut combined = ring_segments.first().unwrap().clone();
        //             let mut last = ring_segments.last().unwrap().clone();
        //             combined.append(&mut last);
        //             ring_segments.push(combined);
        //         }
        //     }
        //     _ => {}
        // }

        // let mut filtered: Vec<Vec<LineElem<T>>> = ring_segments
        //     .iter()
        //     .filter(|segment| segment.len() > 1)
        //     .map(|s| *s)
        //     .collect();
        // self.segments.append(&mut filtered);
    }
}

impl<T> Stream<T> for ClipBase<T>
where
    T: CoordFloat + FloatConst,
{
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        // match self.use_ring {
        //     true => {
        //         self.ring.push(p);
        //         self.ring_sink.point(p, None);
        //     }
        //     false => {
        //         if self.point_visible(p, None) {
        //             let mut sink = self.sink.borrow_mut();
        //             sink.point(p, m);
        //         }
        //     }
        // }
    }
    fn line_start(&mut self) {
        // if self.use_ring_start {
        //     self.ring_start();
        // } else {
        //     // What ghoes here.
        // }
        // // self.clip.point = self.point_line;
        // // self.line.line_start();
    }

    fn line_end(&mut self) {
        // if self.use_ring_end {
        //     self.ring_end();
        // } else {
        //     // put somethignhere.
        // }
    }

    fn polygon_start(&mut self) {
        // self.use_ring = true;
        // self.use_ring_start = true;
        // self.use_ring_end = true;
        // self.segments.clear();
        // self.polygon.clear();
    }

    fn polygon_end(&mut self) {
        // self.use_ring = false;
        // self.use_ring_start = false;
        // self.use_ring_end = false;
        // // segments = merge(segments);
        // let start_inside = contains(&self.polygon, &self.start);
        // let mut sink = self.sink.borrow_mut();
        // if !self.polygon_started {
        //     sink.polygon_start();
        //     self.polygon_started = true;

        //     rejoin(
        //         &self.segments,
        //         self.compare_intersection,
        //         start_inside,
        //         self.interpolate,
        //         self.sink,
        //     );
        // } else if start_inside {
        //     if !self.polygon_started {
        //         sink.polygon_start();
        //         self.polygon_started = true;
        //     }
        //     sink.line_start();
        //     // (self.interpolate)(None, None, 1f64, self.sink);
        //     sink.line_end();
        // }
        // if self.polygon_started {
        //     sink.polygon_end();
        //     self.polygon_started = false;
        // }
        // self.segments.clear();
        // self.polygon.clear();
    }

    fn sphere(&mut self) {
        // let mut sink = self.sink.borrow_mut();
        // sink.polygon_start();
        // sink.line_start();
        // self.interpolate(None, None, T::one(), self.sink);
        // sink.line_end();
        // sink.polygon_end();
    }
}
// impl<T: CoordFloat + FloatConst + 'static> Clip<T> {
//     pub fn gen_stream_processor(
//         point_visible: PointVisibleFn<T>,
//         clip_line: StreamPathResultToCleanProcessor<T>,
//         interpolate: InterpolateFn<T>,
//         start: Coordinate<T>,
//     ) -> StreamPathResultToStreamProcessor<T> {
//         Box::new(move |sink: StreamPathResultNode<T>| {
//             let line = clip_line(sink.clone());

//             let ring_buffer_node = ClipBuffer::gen_stream_result_node();
//             // let ring_buffer_node: StreamSimpleNode<T> = Rc::new(RefCell::new(Box::new(ring_buffer)));

//             let ring_sink = clip_line(ring_buffer_node);

//             // Intersections are sorted along the clip edge. For both antimeridian cutting
//             // and circle clipPIng, the same comparison is used.
//             let compare_intersection: CompareIntersectionFn<T> =
//                 Rc::new(Box::new(|a: Ci<T>, b: Ci<T>| -> T {
//                     let a_dashed = a.x;
//                     let part1 = match a_dashed.x < T::zero() {
//                         true => a_dashed.y - T::FRAC_PI_2() - T::epsilon(),
//                         false => T::FRAC_PI_2() - a_dashed.y,
//                     };
//                     let b_dashed = b.x;
//                     let part2 = match b_dashed.x < T::zero() {
//                         true => b_dashed.y - T::FRAC_PI_2() - T::epsilon(),
//                         false => T::FRAC_PI_2() - b_dashed.y,
//                     };

//                     return part1 - part2;
//                 }));

//             Rc::new(RefCell::new(Box::new(Self {
//                 use_ring: false,
//                 use_ring_start: false,
//                 use_ring_end: false,
//                 interpolate: interpolate.clone(),
//                 compare_intersection,
//                 line,
//                 point_visible: point_visible.clone(),
//                 polygon: Vec::new(),
//                 polygon_started: false,
//                 ring: Vec::new(), // Javascript leaves this undefined here.
//                 ring_buffer_node: ring_buffer_node.clone(),
//                 ring_sink,
//                 segments: Vec::new(),
//                 sink: sink.clone(),
//                 start: start.clone(),
//             })))
//         })
//     }
// }

use crate::stream::StreamSimple;
struct ClipTraitIdentity {}
impl<T> StreamSimple<T> for ClipTraitIdentity where T: CoordFloat + FloatConst {}
impl<T> StreamClipTrait<T> for ClipTraitIdentity
where
    T: CoordFloat + FloatConst,
{
    fn point_visible(&self, p: Coordinate<T>, _z: Option<u8>) -> bool {
        false
    }
    fn interpolate(
        &self,
        from: Option<Coordinate<T>>,
        to: Option<Coordinate<T>>,
        direction: T,
        stream: StreamSimpleNode<T>,
    ) {
        // Dummy function.
    }
}
impl<T> StreamInTrait<T> for ClipTraitIdentity where T: CoordFloat + FloatConst {}
impl<T> StreamPathResult<T> for ClipTraitIdentity where T: CoordFloat + FloatConst {}
impl<T> Stream<T> for ClipTraitIdentity where T: CoordFloat + FloatConst {}
impl<T> PathResult<T> for ClipTraitIdentity where T: CoordFloat + FloatConst {}
