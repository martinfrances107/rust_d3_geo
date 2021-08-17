pub mod none;
pub mod resample;

use crate::Transform;
use std::fmt::Debug;
use std::fmt::Display;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::resample::none::None;
use super::resample::resample::Resample;
use super::stream_node::StreamNode;
use super::stream_node_factory::StreamNodeFactory;
use super::Raw as ProjectionRaw;

#[derive(Debug)]
pub enum ResampleEnum<PR, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    RN(None<PR, T>),
    R(Resample<PR, T>),
}

impl<PR, T> Default for ResampleEnum<PR, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    #[inline]
    fn default() -> Self {
        ResampleEnum::RN(None::<PR, T>::default())
    }
}

impl<PR, T> Clone for ResampleEnum<PR, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            ResampleEnum::RN(rn) => ResampleEnum::RN(*rn),
            ResampleEnum::R(r) => ResampleEnum::R(*r),
        }
    }
}
impl<'a, PR, SINK, T> Stream for StreamNode<ResampleEnum<PR, T>, SINK, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    SINK: Stream<T = T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    type T = T;

    fn sphere(&mut self) {
        todo!("must fix.");
    }
    fn polygon_start(&mut self) {
        todo!("must fix.");
    }
    fn polygon_end(&mut self) {
        todo!("must fix.");
    }
    fn point(&mut self, _p: &Coordinate<T>, _m: Option<u8>) {
        todo!("must fix.");
    }
    fn line_start(&mut self) {
        todo!("must fix.");
    }
    fn line_end(&mut self) {
        todo!("must fix.");
    }
}

#[inline]
pub fn gen_resample_factory<PR, SINK, T>(
    projection_raw: PR,
    delta2: T,
) -> StreamNodeFactory<ResampleEnum<PR, T>, SINK, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    SINK: Stream<T = T>,
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    if delta2.is_zero() {
        StreamNodeFactory::new(ResampleEnum::RN(None::new(projection_raw)))
    } else {
        StreamNodeFactory::new(ResampleEnum::R(Resample::new(projection_raw)))
    }
}
