/// Resample None.
pub mod none;
/// Resamples
pub mod resample;

/// Factory Method.
pub mod stream_node_resample_factory;

use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::stream_node::StreamNode;
use super::Raw as ProjectionRaw;

use none::None;
use resample::Resample;

/// A return type which contains the
/// generated resample node.
#[derive(Debug, Clone)]
pub enum ResampleNode<EP, PR, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    PR: ProjectionRaw<T>,
    SINK: Stream<EP = EP, T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// Resample None.
    RN(StreamNode<EP, None<PR, T>, SINK, T>),
    /// A Resample Node.
    R(StreamNode<EP, Resample<PR, T>, SINK, T>),
}

impl<'a, EP, PR, SINK, T> Stream for ResampleNode<EP, PR, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    PR: ProjectionRaw<T>,
    SINK: Stream<EP = EP, T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn get_endpoint(self) -> Self::EP {
        match self {
            ResampleNode::RN(n) => n.get_endpoint(),
            ResampleNode::R(r) => r.get_endpoint(),
        }
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self {
            ResampleNode::RN(n) => n.point(p, m),
            ResampleNode::R(r) => r.point(p, m),
        };
    }

    #[inline]
    fn polygon_start(&mut self) {
        match self {
            ResampleNode::RN(n) => n.polygon_start(),
            ResampleNode::R(r) => r.polygon_start(),
        };
    }

    #[inline]
    fn polygon_end(&mut self) {
        match self {
            ResampleNode::RN(n) => n.polygon_end(),
            ResampleNode::R(r) => r.polygon_end(),
        };
    }

    #[inline]
    fn line_start(&mut self) {
        match self {
            ResampleNode::RN(n) => n.line_start(),
            ResampleNode::R(r) => r.line_start(),
        };
    }

    #[inline]
    fn line_end(&mut self) {
        match self {
            ResampleNode::RN(n) => n.line_end(),
            ResampleNode::R(r) => r.line_end(),
        };
    }

    #[inline]
    fn sphere(&mut self) {
        match self {
            ResampleNode::RN(n) => n.sphere(),
            ResampleNode::R(r) => r.sphere(),
        };
    }
}
