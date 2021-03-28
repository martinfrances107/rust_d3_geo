use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::ops::AddAssign;

use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::clip::Clip;
use crate::clip::ClipRaw;
use crate::compose::Compose;
use crate::stream::Stream;
use crate::stream::StreamDst;
use crate::Transform;

#[derive(Debug)]
pub struct ResampleNone<T>
where
    T: CoordFloat + Default + FloatConst,
{
    project: Compose<T>,
    /// Box to prevent infinite recusion.
    pub stream: Box<Clip<T>>,
}

impl<T> Clone for ResampleNone<T>
where
    T: CoordFloat + Default + FloatConst,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            project: self.project.clone(),
            stream: self.stream.clone(),
        }
    }
}

impl<T: CoordFloat + Default + FloatConst> ResampleNone<T> {
    #[inline]
    pub fn new(project: Compose<T>) -> Self {
        Self {
            project: project,
            stream: Box::new(Clip::new(
                ClipRaw::Antimeridian(ClipAntimeridian::default()),
                Coordinate::default(),
            )), // stub value
        }
    }
}

impl<T: CoordFloat + Default + FloatConst> ResampleNone<T> {
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
        let p = p.clone();
        let project = &self.project;
        let t = project.transform(&p);
        self.stream.point(&t, m);
    }
}
