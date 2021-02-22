use crate::path::PathResultEnum;
use crate::stream::StreamSimpleNode;
use crate::stream::{Stream, StreamPathResultNode};
use crate::{path::PathResult, stream::StreamInTrait};

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;
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

impl<T: CoordFloat + FloatConst + 'static> ClipBuffer<T> {
    /// Generate a new stream node.
    #[inline]
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            line: None,
        }
    }

    #[inline]
    pub fn gen_node() -> StreamPathResultNode<T>
    where
        T: CoordFloat + FloatConst,
    {
        Rc::new(RefCell::new(Self::new()))
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
impl<T> StreamInTrait<T> for ClipBuffer<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_in(&mut self, _stream: StreamSimpleNode<T>) {
        panic!("Should I call stream_in on a buffer!");
    }
}

use crate::stream::StreamPathResult;
impl<T> StreamPathResult<T> for ClipBuffer<T> where T: CoordFloat + FloatConst {}
impl<'a, T: CoordFloat + FloatConst> Stream for ClipBuffer<T> {
    type C = Coordinate<T>;
    #[inline]
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        match self.line.clone() {
            Some(mut line) => {
                line.push(LineElem { p, m });
            }
            None => {
                panic!("Cannot push to undefined line.");
            }
        }
    }

    fn line_start(&mut self) {
        let line = Vec::new();
        self.line = Some(line.clone());
        self.lines.push(line);
    }
}
