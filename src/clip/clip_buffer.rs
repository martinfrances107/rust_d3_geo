use std::collections::VecDeque;
use std::fmt::Display;
// use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::path::PathResult;
use crate::path::PathResultEnum;
// use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;

use super::line_elem::LineElem;

#[derive(Clone, Debug)]
pub struct ClipBuffer<T>
where
    T: CoordFloat,
{
    // pd: PhantomData<SD>,
    // line: Vec<LineElem<T>>,
    lines: VecDeque<Vec<LineElem<T>>>,
}

impl<T> Default for ClipBuffer<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            lines: VecDeque::default(),
        }
    }
}

impl<T> PathResult for ClipBuffer<T>
where
    // SD: StreamDst,
    T: CoordFloat,
{
    type Out = Option<PathResultEnum<T>>;
    fn result(&mut self) -> Option<PathResultEnum<T>> {
        let result = self.lines.clone();
        self.lines.clear();
        return Some(PathResultEnum::ClipBufferOutput(result));
    }
}

impl<T> Stream for ClipBuffer<T>
where
    // SD: StreamDst,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SC = Coordinate<T>;
    // type ST = T;
    // type SD = SD;
    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
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
    }
    fn polygon_start(&mut self) {
        println!("clipBuffer polygon_start()");
    }
    fn polygon_end(&mut self) {
        println!("clipBuffer polygon_end()");
    }
    // fn get_dst(&self) -> Self {
    //     todo!("ClipBuffer get_dst() should never be called.");
    // }
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
