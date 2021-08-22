use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::Line;
use crate::clip::PointVisible;
use crate::stream::Stream;
use crate::Transform;

use projection::Projection;
use stream_node::StreamNode;

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
pub trait Raw<T>: Clone + Copy + Default + Transform<T = T>
where
    <Self as Transform>::T: CoordFloat,
{
    type T;
}

trait Builder
where
    <Self as Builder>::Drain: Stream<T = <Self as Builder>::T>,
    <Self as Builder>::L: Line,
    <Self as Builder>::PR: Raw<Self::T>,
    <Self as Builder>::PV: PointVisible<T = Self::T>,
    <Self as Builder>::T: CoordFloat + FloatConst,
{
    type Drain;
    type L;
    type PR;
    type PV;
    type T;
    fn build(s: Self::PR) -> Projection<Self::Drain, Self::L, Self::PR, Self::PV, Self::T>;
}

/// Generates elements of the  projection stream pipeline.
pub trait NodeFactory
where
    <Self as NodeFactory>::T: CoordFloat,
{
    type Raw;
    type Sink;
    type T;
    type Node;
    fn generate(&self, sink: Rc<RefCell<Self::Sink>>) -> Self::Node;

    // -> Self::Node<Self::Raw, Self::Sink, Self::T>;
}
