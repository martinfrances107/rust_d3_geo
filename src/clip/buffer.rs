use std::collections::VecDeque;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
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
    lines: VecDeque<Vec<LineElem<T>>>,
}

impl<T: CoordFloat + FloatConst> ClipBuffer<T> {
    // #[inline]
    // pub fn stream_in(&mut self, _stream: Box<dyn Stream<T, C = Coordinate<T>>>) {
    //     panic!("Should I call stream_in on a buffer!");
    // }

    fn rejoin(&mut self) {
        if self.lines.len() > 1 {
            let pb = [
                self.lines.pop_back().unwrap(),
                self.lines.pop_front().unwrap(),
            ]
            .concat();
            self.lines.push_front(pb);
        }
    }
}

impl<T: CoordFloat> PathResult for ClipBuffer<T> {
    type Out = Option<PathResultEnum<T>>;
    fn result(&mut self) -> Option<PathResultEnum<T>> {
        let result = self.lines.clone();
        self.lines.clear();
        return Some(PathResultEnum::ClipBufferOutput(result));
    }
}

impl<T: AddAssign + CoordFloat + Default + FloatConst> Stream<T> for ClipBuffer<T> {
    type C = Coordinate<T>;
    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        match self.lines.back_mut() {
            Some(line) => {
                line.push(LineElem { p: *p, m });
            }
            None => panic!("buffers: lines was not properly initialised."),
        }
    }

    fn sphere(&mut self) {}
    fn line_end(&mut self) {}
    #[inline]
    fn line_start(&mut self) {
        self.lines.push_back(Vec::new());
    }
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
    fn get_dst(&self) -> StreamDst<T> {
        todo!("ClipBuffer get_dst() should never be called.");
    }
}
