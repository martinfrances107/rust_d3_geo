use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::clip_sink_enum::ClipSinkEnum;
use crate::compose::Compose;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::Transform;

#[derive(Clone, Debug)]
pub struct ResampleNone<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    project: Compose<T>,
    /// Box to prevent infinite recusion.
    pub stream: Box<ClipSinkEnum<T>>,
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> ResampleNone<T> {
    #[inline]
    pub fn new(project: Compose<T>) -> Self {
        Self {
            project: project,
            stream: Box::new(ClipSinkEnum::Blank), // stub value
        }
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> ResampleNone<T> {
    #[inline]
    pub fn stream_in(&mut self, stream: ClipSinkEnum<T>) {
        self.stream = Box::new(stream);
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Stream<T>
    for ResampleNone<T>
{
    type C = Coordinate<T>;
    #[inline]
    fn get_dst(&self) -> StreamDst<T> {
        self.stream.get_dst()
    }

    #[inline]
    fn sphere(&mut self) {
        self.stream.sphere()
    }

    #[inline]
    fn line_start(&mut self) {
        self.stream.line_start()
    }

    #[inline]
    fn line_end(&mut self) {
        self.stream.line_end()
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.stream.polygon_start()
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.stream.polygon_end()
    }

    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        let t = &self.project.transform(&p);
        self.stream.point(&t, m);
    }
}
