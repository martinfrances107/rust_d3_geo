use crate::path::PathResult;
use crate::path::PathResultEnum;
use crate::stream::Stream;
use crate::stream::StreamDst;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

#[derive(Clone, Copy, Debug)]
pub struct LineElem<T: CoordFloat> {
    pub p: Coordinate<T>,
    pub m: Option<u8>,
}
#[derive(Clone, Debug, Default)]
pub struct ClipBuffer<T: CoordFloat> {
    lines: Vec<Vec<LineElem<T>>>,
    line: Option<Vec<LineElem<T>>>,
}

impl<T: CoordFloat + FloatConst> ClipBuffer<T> {
    /// Generate a new stream node.
    // #[inline]
    // pub fn new() -> Self {
    //     Self {
    //         lines: Vec::new(),
    //         line: None,
    //     }
    // }

    fn rejoin(&mut self) {
        if self.lines.len() > 1 {
            // Shift from the top end.
            let lines_shift = self.lines.remove(0);
            // Pop from the bottom end.
            let lines_pop = self.lines.pop().unwrap_or(Vec::new());
            let join = [lines_pop, lines_shift].concat();
            self.lines.push(join);
        }
    }
}

impl<T: CoordFloat> PathResult for ClipBuffer<T> {
    type Out = Option<PathResultEnum<T>>;
    fn result(&mut self) -> Option<PathResultEnum<T>> {
        let result = self.lines.clone();
        self.lines.clear();
        self.line = None;
        // let result = &self.lines;
        // return result.to_vec();
        // TODO must fix this!!
        return Some(PathResultEnum::ClipBufferOutput(result.to_vec()));
    }
}
impl<T> ClipBuffer<T>
where
    T: CoordFloat + FloatConst,
{
    pub fn stream_in(&mut self, _stream: Box<dyn Stream<T, C = Coordinate<T>>>) {
        panic!("Should I call stream_in on a buffer!");
    }
}

impl<T: CoordFloat + Default + FloatConst> Stream<T> for ClipBuffer<T> {
    type C = Coordinate<T>;
    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        match self.line.clone() {
            Some(mut line) => {
                line.push(LineElem { p: *p, m });
            }
            None => {
                panic!("Cannot push to undefined line.");
            }
        }
    }

    fn sphere(&mut self) {}
    fn line_end(&mut self) {}
    fn line_start(&mut self) {
        let line = Vec::new();
        self.line = Some(line.clone());
        self.lines.push(line);
    }
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
    fn get_dst(&self) -> StreamDst<T> {
        todo!("is this ever called.");
    }
}
