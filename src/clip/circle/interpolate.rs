use crate::circle::generate::generate;
use crate::clip::InterpolateRaw;
use crate::stream::Stream;
// use crate::clip::clip::Clip;
use crate::clip::InterpolateTrait;
use crate::projection::stream_node::StreamNode;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use std::fmt::Display;
use std::ops::AddAssign;

/// Circle Interpolate.
#[derive(Clone, Debug)]
pub struct Interpolate<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    radius: T,
    delta: T,
}

impl<T> InterpolateRaw for Interpolate<T> where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst
{
}

impl<T> Interpolate<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn new(radius: T) -> Self {
        Self {
            radius,
            delta: T::from(6.0_f64 * std::f64::consts::PI / 180.0_f64).unwrap(),
        }
    }
}

// use super::line::Line;
// use super::pv::PV;
impl<SINK, T> InterpolateTrait for StreamNode<Interpolate<T>, SINK, T>
where
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type IC = Coordinate<T>;
    type IT = T;

    #[inline]
    fn interpolate(
        &mut self,
        from: Option<Coordinate<T>>,
        to: Option<Coordinate<T>>,
        direction: T,
    ) {
        generate(
            self.sink,
            self.raw.radius,
            self.raw.delta,
            direction,
            from,
            to,
        )
    }
}
