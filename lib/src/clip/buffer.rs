use std::collections::VecDeque;

use geo::CoordFloat;
use geo_types::Coord;

use crate::path::Result;
use crate::stream::Stream;

use super::line_elem::LineElem;

/// Buffer is a path endpoint.
///
/// Stored data can be extracted via [`Buffer::result()`]
#[derive(Clone, Debug)]
pub struct Buffer<T>
where
    T: CoordFloat,
{
    pub lines: VecDeque<Vec<LineElem<T>>>,
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
        let mut result = VecDeque::new();
        core::mem::swap(&mut result, &mut self.lines);
        result
    }
}

impl<T> Buffer<T>
where
    T: CoordFloat,
{
    /// Stitch first and last elements together.
    pub(super) fn rejoin(&mut self) {
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
            self.lines.push_back(combined);
        }
    }
}

impl<T> Stream for Buffer<T>
where
    T: CoordFloat,
{
    type T = T;
    type EP = Self;

    fn endpoint(&mut self) -> &mut Self::EP {
        self
    }

    #[inline]
    fn point(&mut self, p: &Coord<T>, m: Option<u8>) {
        self.lines.back_mut().map_or_else(
            || panic!("buffers: lines was not properly initialised."),
            |line| {
                line.push(LineElem { p: *p, m });
            },
        );
    }

    #[inline]
    fn line_start(&mut self) {
        self.lines.push_back(vec![]);
    }
}
