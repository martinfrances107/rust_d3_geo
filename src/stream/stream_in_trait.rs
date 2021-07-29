use super::Stream;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::Float;
use num_traits::FloatConst;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

pub trait StreamIn {
    type SInput;
    fn stream_in(&mut self, stream: Self::SInput);
}

pub trait StreamDynIn {
    type SDIT;
    fn stream_in(&mut self, stream: Box<dyn Stream<SC = Coordinate<Self::SDIT>>>)
    where
        <Self as StreamDynIn>::SDIT: AddAssign
            + AsPrimitive<<Self as StreamDynIn>::SDIT>
            + Debug
            + Display
            + Float
            + FloatConst;
}
pub trait StreamCombo: Stream + StreamIn {}
