use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::ops::AddAssign;

use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::buffer::LineElem;
use crate::clip::clip::Clip;
use crate::clip::clip_raw::ClipRaw;
use crate::compose::Compose;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::Transform;

#[derive(Debug)]
pub struct ResampleNone<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    project: Compose<T>,
    /// Box to prevent infinite recusion.
    pub stream: Box<Clip<T>>,
}

impl<T> Clone for ResampleNone<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            project: self.project.clone(),
            stream: self.stream.clone(),
        }
    }
}

impl<T: AddAssign + CoordFloat + Default + FloatConst> ResampleNone<T> {
    #[inline]
    pub fn new(project: Compose<T>) -> Self {
        Self {
            project: project,
            stream: Box::new(Clip::new(
                ClipRaw::Antimeridian(ClipAntimeridian::default()),
                LineElem {
                    p: Coordinate::default(),
                    m: None,
                },
            )), // stub value
        }
    }
}

impl<T: AddAssign + CoordFloat + Default + FloatConst> ResampleNone<T> {
    #[inline]
    pub fn stream_in(&mut self, stream: Clip<T>) {
        self.stream = Box::new(stream);
    }
}

impl<T: AddAssign + CoordFloat + Default + FloatConst> Stream<T> for ResampleNone<T> {
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
        println!("resample_none point()");
        let t = &self.project.transform(&p);
        self.stream.point(&t, m);
    }
}
