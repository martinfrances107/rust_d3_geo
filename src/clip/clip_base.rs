use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// use crate::path::PathResultEnum;
// use crate::stream::stream_clip_line_node_stub::StreamClipLineNodeStub;
// use crate::stream::stream_path_result_node_stub::StreamPathResultNodeStub;
use crate::stream::Stream;
// use crate::stream::StreamClipLine;
// use super::antimeridian::ClipAntimeridian;
// use super::circle::ClipCircle;
// use super::ClipRaw;
use super::LineEnum;
use crate::stream::StreamClone;
// use crate::stream::StreamPathResult;

// use super::antimeridian::ClipAntimeridian;
use super::antimeridian::line::Line as AntimeridianLine;
use super::buffer::ClipBuffer;
use super::buffer::LineElem;
// use super::circle::ClipCircle;
// use super::ClipTraitRaw;
use super::clip::ClipSinkEnum;

#[derive(Clone)]
pub struct ClipBase<T: CoordFloat + FloatConst + Default + 'static> {
    pub line: LineEnum<T>,
    pub polygon_started: bool,
    pub polygon: Vec<Vec<Coordinate<T>>>,
    pub ring: Vec<Coordinate<T>>,
    pub ring_sink: LineEnum<T>,
    pub ring_buffer: ClipBuffer<T>,
    // pub ring_sink_node: Box<dyn StreamClipLine<C = Coordinate<T>, BitCB = ClipBuffer<T>>>,
    // pub ring_sink_node: ClipRaw<T>,
    pub segments: Vec<Vec<LineElem<T>>>,
    pub start: Coordinate<T>,
    pub use_ring: bool,
    pub use_ring_end: bool,
    pub use_ring_start: bool,
    // pub sink: Box<dyn StreamPathResult<C = Coordinate<T>, Out = Option<PathResultEnum<T>>>>,
    pub sink: ClipSinkEnum<T>,
}

// impl<T> Clone for ClipBase<T>
// where
//     T: CoordFloat + FloatConst + Default,
// {
//     fn clone(&self) -> Self {
//         Self {
//             // sink: self.sink.box_clone(),
//             ..*self
//         }
//     }
// }

use crate::projection::resample::resample::Resample;
use crate::projection::resample::ResampleEnum;

impl<T> Default for ClipBase<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn default() -> Self {
        Self {
            // Must be overrided.
            // line_node: Box::new(StreamClipLineNodeStub::default()),
            // clip_buffer: Box::new(StreamClipLineNodeStub::default()),
            line: LineEnum::Antimeridian(AntimeridianLine::default()),
            polygon_started: false,
            polygon: vec![vec![]],
            ring: vec![],
            // ring_buffer_node: Box::new(StreamPathResultNodeStub::default()),
            ring_buffer: ClipBuffer::default(),
            // clip_buffer: ClipBuffer::default(),
            // ring_sink_node: Box::new(StreamClipLineNodeStub::default()),
            ring_sink: LineEnum::Antimeridian(AntimeridianLine::default()),
            segments: vec![vec![]],
            use_ring: false,
            use_ring_end: false,
            use_ring_start: false,
            // sink: Box::new(StreamPathResultNodeStub::default()),
            sink: ClipSinkEnum::Resample(ResampleEnum::R(Resample::default())), // stub value
            start: Coordinate {
                x: -T::PI(),
                y: -T::FRAC_PI_2(),
            },
        }
    }
}

impl<T> ClipBase<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn point_ring(&mut self, p: Coordinate<T>, _m: Option<u8>) {
        self.ring.push(p);
        self.ring_sink.point(p, None);
    }

    fn ring_start(&mut self) {
        self.ring_sink.line_start();
        self.ring = Vec::new();
    }

    fn ring_end(&mut self) {
        self.point_ring(self.ring[0], None);
        self.ring_sink.line_end();

        // let clean = self.ring_sink.clean();
        // let mut ring_buffer = self.ring_buffer_node.borrow_mut();
        // let ring_segments = match self.ring_buffer.result() {
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

impl<T> StreamClone for ClipBase<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        // Box::new(*self.clone())
        panic!("Make clip base clonable")
    }
}

// impl<T> StreamClipTrait for ClipBase<T>
// where
//     T: CoordFloat + FloatConst + 'static,
// {
//     type SctOC = Option<Coordinate<T>>;
//     type SctStream = StreamSimpleNode<T>;
//     type SctT = T;
//     type SctCi = CompareIntersection<T>;
// }

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
