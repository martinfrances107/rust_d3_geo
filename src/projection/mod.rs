use crate::clip::LineRaw;
use crate::clip::PointVisible;
use crate::projection::projection::Projection;
use crate::stream::Stream;
use crate::Transform;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::Float;
use num_traits::FloatConst;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use crate::projection::stream_node::StreamNode;

pub mod azimuthal;
pub mod azimuthal_equal_area;
pub mod builder;
pub mod builder_trait;
pub mod center;
pub mod clip_extent;
pub mod equirectangular;
pub mod gnomic;
pub mod mecator;
pub mod orthographic;
pub mod projection;
pub mod projection_equal;
pub mod scale;
pub mod scale_translate;
pub mod scale_translate_rotate;
pub mod stereographic;
pub mod stream_node;
pub mod stream_node_factory;
pub mod stream_transform;
pub mod stream_transform_radians;
pub mod translate;

mod fit;
mod resample;

/// Projection Raw.
pub trait Raw: Clone + Copy + Default + Transform<C = Coordinate<Self::T>>
where
    <Self as Raw>::T: Debug + Display + Float + FloatConst,
{
    type T;
}

trait Builder
where
    <Self as Builder>::Drain: Stream<SC = Coordinate<<Self as Builder>::T>>,
    <Self as Builder>::L: LineRaw,
    <Self as Builder>::PR: Raw<T = Self::T>,
    <Self as Builder>::PV: PointVisible<T = Self::T>,
    <Self as Builder>::T: AddAssign
        + AsPrimitive<<Self as Builder>::T>
        + Default
        + Debug
        + Display
        + Float
        + FloatConst,
{
    type Drain;
    type L;
    type PR;
    type PV;
    type T;
    fn build(s: Self::PR) -> Projection<Self::Drain, Self::L, Self::PR, Self::PV, Self::T>;
}

/// Generates elements of the  projection stream pipeline.
pub trait NodeFactory {
    type Raw;
    type Sink;
    type T;
    fn generate(&self, sink: Rc<RefCell<Self::Sink>>)
        -> StreamNode<Self::Raw, Self::Sink, Self::T>;
}
