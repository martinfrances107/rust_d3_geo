use crate::clip::InterpolateRaw;
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
// use std::rc::Rc;

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
pub mod resample;
pub mod scale;
pub mod scale_translate;
pub mod scale_translate_rotate;
pub mod stereographic;
// used by clip
pub mod stream_node;
// use by clip
pub mod stream_node_factory;
pub mod stream_transform;
pub mod stream_transform_radians;
pub mod translate;

// Internal to projection.
mod fit;

/// Projection Raw.
pub trait Raw: Clone + Copy + Default + Transform<C = Coordinate<Self::T>>
where
    <Self as Raw>::T: Debug + Display + Float + FloatConst,
{
    type T;
}

trait Builder {
    type Drain;
    type I;
    type L;
    type PR;
    type PV;
    type T;
    fn build<'a>(
        s: &'a Self::PR,
    ) -> Projection<Self::Drain, Self::I, Self::L, Self::PR, Self::PV, Self::T>
    where
        <Self as Builder>::Drain: Stream<SC = Coordinate<<Self as Builder>::T>>,
        <Self as Builder>::I: InterpolateRaw,
        <Self as Builder>::L: LineRaw,
        <Self as Builder>::PR: Raw<T = Self::T>,
        <Self as Builder>::PV: PointVisible,
        <Self as Builder>::T: AddAssign
            + AsPrimitive<<Self as Builder>::T>
            + Default
            + Debug
            + Display
            + Float
            + FloatConst;
}

/// Generates elements of the  projection stream pipeline.
pub trait NodeFactory {
    type Raw;
    type Sink;
    type T;
    fn generate(
        &self,
        sink: Rc<RefCell<Self::Sink>>,
    ) -> Rc<RefCell<StreamNode<Self::Raw, Self::Sink, Self::T>>>;
    // where
    //     <Self as NodeFactory>::T: AddAssign
    //         + AsPrimitive<<Self as NodeFactory>::T>
    //         + Default
    //         + Debug
    //         + Display
    //         + Float
    //         + FloatConst;
}
