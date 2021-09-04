use crate::clip::clip::Clip;
use crate::projection::resample::ResampleNode;
use crate::projection::stream_node_factory::StreamNodeFactory;
use crate::rotation::rotate_radians::RotateRadians;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::Line;
use crate::clip::PointVisible;
use crate::stream::Stream;
use crate::Transform;

use projection::Projection;
use stream_node::StreamNode;

/// Helper functions.
pub mod azimuthal;
/// The raw projection.
pub mod azimuthal_equal_area;
/// The projection builder.
pub mod builder;
pub mod builder_trait;
/// A projection builder sub trait.
pub mod center;
/// A projection builder sub trait.
pub mod clip_extent;
/// The raw projection.
pub mod equirectangular;
/// The raw projection.
pub mod gnomic;
/// The raw projection.
pub mod mercator;
/// Mecators has a specalised builder wrapping the default mecator.
pub mod mercator_builder;
/// The raw projection.
pub mod orthographic;
/// Projection object.
pub mod projection;
/// Debug and test helper function.
pub mod projection_equal;
/// A projection builder subtrait.
pub mod scale;
/// The raw projection.
pub mod stereographic;
/// Scale translate and rotate.
pub mod str;
/// Stream node pipeline stage.
pub mod stream_node;
/// Generates stream node objects.
pub mod stream_node_factory;
/// A stream node pipeline stage.
pub mod stream_transform_radians;
/// A projection builder sub trait.
pub mod translate;

mod fit;
mod resample;

pub(crate) type RotateFactory<DRAIN, L, PR, PV, T> = StreamNodeFactory<
    RotateRadians<T>,
    StreamNode<Clip<L, PV, ResampleNode<PR, DRAIN, T>, T>, ResampleNode<PR, DRAIN, T>, T>,
    T,
>;

/// Projection Raw.
pub trait Raw<T>: Clone + Debug + Default + Transform<T = T>
where
    <Self as Transform>::T: CoordFloat,
{
    /// f32 or f64.
    type T;
    /// The default builder.
    type Builder;
    /// Constructs the default projection builder.
    fn builder() -> Self::Builder;
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

/// Generates elements of the projection stream pipeline.
pub trait NodeFactory
where
    <Self as NodeFactory>::T: CoordFloat,
{
    /// The resultant node type.
    type Node;
    /// The downstream node.
    type Sink;
    /// f32 or f64.
    type T;

    /// Combine the sink with the proto-node and output a StreamNode.
    fn generate(&self, sink: Rc<RefCell<Self::Sink>>) -> Self::Node;
}
