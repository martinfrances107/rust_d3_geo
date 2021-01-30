use crate::path::PathResult;
use crate::path::PathResultEnum;
use crate::stream::Stream;
use crate::stream::StreamNode;
use geo::CoordFloat;
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Default)]
struct LineTuple<T: CoordFloat> {
    x: T,
    y: T,
    m: Option<u8>,
}
#[derive(Debug, Default)]
pub struct ClipBuffer<T: CoordFloat> {
    lines: Vec<Vec<LineTuple<T>>>,
    line: Vec<LineTuple<T>>,
}

impl<T: CoordFloat + FloatConst + 'static> ClipBuffer<T> {
    /// Generate a new stream node.
    #[inline]
    pub fn new() -> StreamNode<T> {
        Rc::new(RefCell::new(Box::new(Self {
            lines: Vec::new(),
            line: Vec::new(),
        })))
    }

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

impl<T: CoordFloat> PathResult<T> for ClipBuffer<T> {
    fn result(&mut self) -> PathResultEnum<T> {
        self.lines.clear();
        self.line.clear();
        let result = &self.lines;
        // return result.to_vec();
        // TODO must fix this!!
        return PathResultEnum::Path();
    }
}

impl<'a, T: CoordFloat + FloatConst> Stream<T> for ClipBuffer<T> {
    #[inline]
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        self.line.push(LineTuple { x, y, m });
    }

    fn line_start(&mut self) {
        self.line.clear();
        // self.lines.push(self.line);
    }
}
