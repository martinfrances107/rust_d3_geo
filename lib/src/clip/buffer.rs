use std::collections::VecDeque;

use geo::CoordFloat;
use geo::Coordinate;

use crate::path::Result;
use crate::stream::Stream;

use super::line_elem::LineElem;

/// Buffer is a pipeline terminating object ( a drain object ).
///
/// Stored data can be extracted via ::result()
#[derive(Clone, Debug)]
pub struct Buffer<T>
where
    T: CoordFloat,
{
    lines: VecDeque<Vec<LineElem<T>>>,
}

impl<T> Default for Buffer<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            lines: VecDeque::default(),
        }
    }
}

impl<T> Result for Buffer<T>
where
    T: CoordFloat,
{
    type Out = VecDeque<Vec<LineElem<T>>>;

    fn result(&mut self) -> Self::Out {
        let result = self.lines.clone();
        self.lines.clear();
        result
    }
}

impl<T> Buffer<T>
where
    T: CoordFloat,
{
    /// Stitch first and last elements together.
    pub fn rejoin(&mut self) {
        if self.lines.len() > 1 {
            let line_last = self
                .lines
                .pop_back()
                .unwrap_or_else(|| Vec::with_capacity(0));
            let line_first = self
                .lines
                .pop_front()
                .unwrap_or_else(|| Vec::with_capacity(0));
            let combined = [line_last, line_first].concat();
            self.lines.push_back(combined)
        }
    }
}

impl<T> Stream for Buffer<T>
where
    T: CoordFloat,
{
    type T = T;
    type EP = Self;

    fn get_endpoint(self) -> Self::EP {
        self
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self.lines.back_mut() {
            Some(line) => {
                line.push(LineElem { p: *p, m });
            }
            None => panic!("buffers: lines was not properly initialised."),
        }
    }

    #[inline]
    fn line_start(&mut self) {
        self.lines.push_back(vec![]);
    }
}
