use std::collections::VecDeque;
use std::fmt::Display;
// use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::path::Result;
use crate::path::ResultEnum;
use crate::stream::Stream;

use super::line_elem::LineElem;

/// Buffer is a pipeline terminating object ( a drain object ).
///
/// Stored data can be extracted via ::result()
#[derive(Clone, Debug)]
pub struct Buffer<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    // line: Vec<LineElem<T>>,
    lines: VecDeque<Vec<LineElem<T>>>,
}

// impl<T> Clone for Buffer<T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     fn clone(&self) -> Buffer<T> {
//         self.clone()
//     }
// }

impl<T> Default for Buffer<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    fn default() -> Self {
        Self {
            lines: VecDeque::default(),
        }
    }
}

impl<T> Result for Buffer<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type Out = Option<ResultEnum<T>>;
    fn result(&mut self) -> Option<ResultEnum<T>> {
        let result = self.lines.clone();
        self.lines.clear();
        Some(ResultEnum::BufferOutput(result))
    }
}

impl<T> Stream for Buffer<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;
    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        println!("Buffer point {:?} {:?}", p, m);
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
        println!("Buffer line_end() -- noop");
    }
    #[inline]
    fn line_start(&mut self) {
        println!("Buffer line_start()");
        // self.line.clear();
        self.lines.push_back(vec![]);
        println!("Buffer line_start lines {:#?}", self.lines);
    }
    fn polygon_start(&mut self) {
        println!("Buffer polygon_start()");
    }
    fn polygon_end(&mut self) {
        println!("Buffer polygon_end()");
    }
    // fn get_dst(&self) -> Self {
    //     todo!("Buffer get_dst() should never be called.");
    // }
}
