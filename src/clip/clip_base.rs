use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// use crate::path::PathResultEnum;
use crate::projection::resample::resample::Resample;
use crate::projection::resample::ResampleEnum;
use crate::stream::Stream;
// use super::antimeridian::ClipAntimeridian;
// use super::circle::ClipCircle;
// use super::ClipRaw;
use super::line_enum::LineEnum;

// use super::antimeridian::ClipAntimeridian;
use super::antimeridian::line::Line as AntimeridianLine;
use super::buffer::ClipBuffer;
use super::buffer::LineElem;
// use super::circle::ClipCircle;
// use super::ClipTraitRaw;
use super::clip_sink_enum::ClipSinkEnum;
use super::line_sink_enum::LineSinkEnum;

#[derive(Clone, Debug)]
pub struct ClipBase<T: AddAssign + CoordFloat + Default + FloatConst> {
    pub line: LineEnum<T>,
    pub polygon_started: bool,
    pub polygon: Vec<Vec<Coordinate<T>>>,
    pub ring: Vec<Coordinate<T>>,
    // pub ring_buffer: LineSinkEnum<T>,
    pub ring_sink: LineEnum<T>,
    // pub ring_buffer: ClipBuffer<T>,
    // pub ring_sink_node: ClipRaw<T>,
    pub segments: Vec<Vec<LineElem<T>>>,
    pub start: Coordinate<T>,
    pub sink: ClipSinkEnum<T>,
}

impl<T> Default for ClipBase<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    fn default() -> Self {
        Self {
            // Must be overrided.
            line: LineEnum::Antimeridian(AntimeridianLine::default()),
            polygon_started: false,
            polygon: vec![vec![]],
            ring: vec![],
            // clip_buffer: ClipBuffer::default(),
            // ring_buffer: LineSinkEnum::CB(ClipBuffer::default()),
            ring_sink: LineEnum::Antimeridian(AntimeridianLine::default()),
            segments: vec![vec![]],
            // sink: ClipSinkEnum::Resample(ResampleEnum::R(Resample::default())), // stub value
            sink: ClipSinkEnum::Blank,
            start: Coordinate {
                x: -T::PI(),
                y: -T::FRAC_PI_2(),
            },
        }
    }
}

// impl<T> ClipBase<T>
// where
//     T: AddAssign + CoordFloat + Default + FloatConst,
// {
//     fn point_ring(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
//         self.ring.push(*p);
//         self.ring_sink.point(p, None);
//     }

//     fn ring_start(&mut self) {
//         self.ring_sink.line_start();
//         self.ring = Vec::new();
//     }

// }

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
