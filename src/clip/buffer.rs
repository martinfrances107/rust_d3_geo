use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::path::PathResult;
use crate::path::PathResultEnum;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;

#[derive(Clone, Copy, Debug)]
pub struct LineElem<T: CoordFloat> {
    pub p: Coordinate<T>,
    pub m: Option<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct ClipBuffer<T: CoordFloat> {
    // line: Vec<LineElem<T>>,
    lines: VecDeque<Vec<LineElem<T>>>,
}

impl<T: CoordFloat> PathResult for ClipBuffer<T> {
    type Out = Option<PathResultEnum<T>>;
    fn result(&mut self) -> Option<PathResultEnum<T>> {
        let result = self.lines.clone();
        self.lines.clear();
        return Some(PathResultEnum::ClipBufferOutput(result));
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Stream<T>
    for ClipBuffer<T>
{
    type C = Coordinate<T>;
    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        println!("ClipBuffer point {:?} {:?}", p, m);
        match self.lines.back_mut() {
            Some(line) => {
                line.push(LineElem { p: *p, m });
            }
            None => panic!("buffers: lines was not properly initialised."),
        }
        // println!("lines {:?}", self.lines);
    }

    fn sphere(&mut self) {}
    fn line_end(&mut self) {
        println!("clipBuffer line_end() -- noop");
    }
    #[inline]
    fn line_start(&mut self) {
        println!("clipBuffer line_start()");
        // self.line.clear();
        self.lines.push_back(vec![]);
        println!("ClipBuffer line_start lines {:#?}", self.lines);
        println!("");
    }
    fn polygon_start(&mut self) {
        println!("clipBuffer polygon_start()");
    }
    fn polygon_end(&mut self) {
        println!("clipBuffer polygon_end()");
    }
    fn get_dst(&self) -> StreamDst<T> {
        todo!("ClipBuffer get_dst() should never be called.");
    }
}
// mpl<T: CoordFloat + FloatConst> ClipBuffer<T> {
//     fn rejoin(&mut self) {
//         if self.lines.len() > 1 {
//             let pb = [
//                 self.lines.pop_back().unwrap(),
//                 self.lines.pop_front().unwrap(),
//             ]
//             .concat();
//             self.lines.push_front(pb);
//         }
//     }
// }
