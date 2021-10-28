use crate::projection::RotateTransformFactory;
use std::cell::RefCell;
use std::rc::Rc;

use approx::AbsDiffEq;
use derivative::*;
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::clip::clip::Clip;
use crate::clip::post_clip_node::PostClipNode;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::clip::stream_node_post_clip_factory::StreamNodePostClipFactory;
use crate::clip::Line;
use crate::clip::PointVisible;
// use crate::clip::PostClipFn;
use crate::compose::Compose;
use crate::rotation::rotate_radians::RotateRadians;
use crate::stream::Stream;
use crate::Transform;

use super::resample::stream_node_resample_factory::StreamNodeResampleFactory;
use super::resample::ResampleNode;
use super::str::scale_translate_rotate::ScaleTranslateRotate;
use super::stream_node_factory::StreamNodeFactory;
use super::stream_transform_radians::StreamTransformRadians;
use super::NodeFactory;
use super::Raw as ProjectionRaw;
use super::RotateFactory;
use super::StreamNode;

// pub enum StreamOrValueMaybe<T: CoordFloat> {
//     Value(T),
//     SP(Box<dyn Stream<T=T>>),
// }

type TransformRadiansFactory<DRAIN, L, PR, PV, T> = StreamNodeFactory<
    StreamTransformRadians,
    StreamNode<
        RotateRadians<T>,
        StreamNode<
            Clip<L, PV, ResampleNode<PR, PostClipNode<DRAIN, T>, T>, T>,
            ResampleNode<PR, PostClipNode<DRAIN, T>, T>,
            T,
        >,
        T,
    >,
    T,
>;

/// Output of projection.stream().
///
/// use by GeoPath.
pub type ProjectionStreamOutput<DRAIN, L, PR, PV, T> = StreamNode<
    StreamTransformRadians,
    StreamNode<
        RotateRadians<T>,
        StreamNode<
            Clip<L, PV, ResampleNode<PR, PostClipNode<DRAIN, T>, T>, T>,
            ResampleNode<PR, PostClipNode<DRAIN, T>, T>,
            T,
        >,
        T,
    >,
    T,
>;

/// Projection output struct of Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct Projection<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    pub(crate) postclip_factory: StreamNodePostClipFactory<DRAIN, T>,

    pub(crate) resample_factory: StreamNodeResampleFactory<PR, PostClipNode<DRAIN, T>, T>,

    pub(crate) preclip_factory:
        StreamNodeClipFactory<L, PR, PV, ResampleNode<PR, PostClipNode<DRAIN, T>, T>, T>,

    pub(crate) rotate_factory: RotateFactory<DRAIN, L, PR, PV, T>,
    /// Used exclusively by Transform( not stream releated).
    pub rotate_transform: Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,

    pub(crate) rotate_transform_factory: RotateTransformFactory<DRAIN, L, PR, PV, T>,

    pub(crate) transform_radians_factory: TransformRadiansFactory<DRAIN, L, PR, PV, T>,
}

impl<'a, DRAIN, L, PR, PV, T> Projection<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// Connects a DRAIN to projection.
    ///
    /// The Projection Stream Pipeline :-
    ///
    /// StreamTransformRadians -> StreamTransform -> preclip -> resample -> postclip -> DRAIN
    ///
    ///
    /// In javascript stream is used as a property to be removed from the object.
    /// In rust that is a closure.
    pub fn stream(&self, drain: Rc<RefCell<DRAIN>>) -> ProjectionStreamOutput<DRAIN, L, PR, PV, T> {
        // let postclip = (self.postclip)(drain);
        let postclip_node = Rc::new(RefCell::new(self.postclip_factory.generate(drain)));

        let resample_node = Rc::new(RefCell::new(self.resample_factory.generate(postclip_node)));

        let preclip_node = Rc::new(RefCell::new(self.preclip_factory.generate(resample_node)));

        let rotate_node = Rc::new(RefCell::new(self.rotate_factory.generate(preclip_node)));

        // Output stage is a transform_radians node.
        self.transform_radians_factory.generate(rotate_node)
    }
}

impl<'a, DRAIN, L, PR, PV, T> Transform for Projection<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let r = Coordinate {
            x: p.x.to_radians(),
            y: p.y.to_radians(),
        };
        self.rotate_transform.transform(&r)
    }
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let d = self.rotate_transform.invert(p);
        Coordinate {
            x: d.x.to_degrees(),
            y: d.y.to_degrees(),
        }
    }
}
