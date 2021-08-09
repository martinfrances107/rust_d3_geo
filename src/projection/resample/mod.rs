pub mod none;
pub mod resample;

use crate::projection::resample::none::None;
use crate::projection::resample::resample::Resample;
use crate::projection::stream_node::StreamNode;
use crate::projection::stream_node_factory::StreamNodeFactory;
use crate::projection::Raw as ProjectionRaw;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;
// use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;
// use crate::Transform;

trait ResampleTrait {}

#[derive(Debug)]
pub enum ResampleEnum<PR, T>
where
    PR: ProjectionRaw<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Debug + Display + FloatConst,
{
    RN(None<PR, T>),
    R(Resample<PR, T>),
}

impl<PR, T> Default for ResampleEnum<PR, T>
where
    PR: ProjectionRaw<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Debug + Display + FloatConst,
{
    fn default() -> Self {
        ResampleEnum::RN(None::<PR, T>::default())
    }
}

impl<PR, T> Clone for ResampleEnum<PR, T>
where
    PR: ProjectionRaw<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Debug + Display + FloatConst,
{
    fn clone(&self) -> Self {
        match self {
            ResampleEnum::RN(rn) => ResampleEnum::RN(rn.clone()),
            ResampleEnum::R(r) => ResampleEnum::R(r.clone()),
        }
    }
}
impl<'a, PR, SINK, T> Stream for StreamNode<ResampleEnum<PR, T>, SINK, T>
where
    PR: ProjectionRaw<T = T>,
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;

    fn sphere(&mut self) {
        todo!("must fix.");
    }
    fn polygon_start(&mut self) {
        todo!("must fix.");
    }
    fn polygon_end(&mut self) {
        todo!("must fix.");
    }
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        todo!("must fix.");
    }
    fn line_start(&mut self) {
        todo!("must fix.");
    }
    fn line_end(&mut self) {
        todo!("must fix.");
    }
}

pub fn gen_resample_factory<'a, PR, SINK, T>(
    projection_raw: PR,
    delta2: T,
) -> StreamNodeFactory<ResampleEnum<PR, T>, SINK, T>
where
    PR: ProjectionRaw<T = T>,
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    if delta2.is_zero() {
        StreamNodeFactory::new(ResampleEnum::RN(None::new(projection_raw)))
    } else {
        StreamNodeFactory::new(ResampleEnum::R(Resample::new(projection_raw)))
    }
}
