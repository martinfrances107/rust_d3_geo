use approx::AbsDiffEq;
use derivative::*;
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::clip_node::ClipNode;
use crate::clip::post_clip_node::PostClipNode;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::clip::stream_node_post_clip_factory::StreamNodePostClipFactory;
use crate::clip::Line;
use crate::clip::PointVisible;
use crate::compose::Compose;
use crate::projection::RotateTransformFactory;
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

type TransformRadiansFactory<DRAIN, EP, LINE, PR, PV, T> = StreamNodeFactory<
    EP,
    StreamTransformRadians,
    StreamNode<
        EP,
        RotateRadians<T>,
        ClipNode<EP, LINE, PV, ResampleNode<EP, PR, PostClipNode<EP, DRAIN, T>, T>, T>,
        T,
    >,
    T,
>;

/// Output of projection.stream().
///
/// use by GeoPath.
pub type ProjectionStreamOutput<DRAIN, LINE, PR, PV, T> = StreamNode<
    DRAIN,
    StreamTransformRadians,
    StreamNode<
        DRAIN,
        RotateRadians<T>,
        ClipNode<DRAIN, LINE, PV, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>,
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
pub struct Projection<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,

    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    pub(crate) postclip_factory: StreamNodePostClipFactory<DRAIN, T>,

    pub(crate) resample_factory: StreamNodeResampleFactory<PR, PostClipNode<DRAIN, DRAIN, T>, T>,

    pub(crate) preclip_factory: StreamNodeClipFactory<
        DRAIN,
        LINE,
        PR,
        PV,
        ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>,
        T,
    >,

    pub(crate) rotate_factory: RotateFactory<DRAIN, DRAIN, LINE, PR, PV, T>,
    /// Used exclusively by Transform( not stream releated).
    pub rotate_transform: Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,

    pub(crate) rotate_transform_factory: RotateTransformFactory<DRAIN, DRAIN, LINE, PR, PV, T>,

    pub(crate) transform_radians_factory: TransformRadiansFactory<DRAIN, DRAIN, LINE, PR, PV, T>,

    pub(crate) cache: Option<(DRAIN, ProjectionStreamOutput<DRAIN, LINE, PR, PV, T>)>,
}

impl<'a, DRAIN, LINE, PR, PV, T> Projection<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + PartialEq<DRAIN>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    ProjectionStreamOutput<DRAIN, LINE, PR, PV, T>: Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
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
    pub fn stream(&mut self, drain: DRAIN) -> ProjectionStreamOutput<DRAIN, LINE, PR, PV, T> {
        if let Some((cache_drain, output)) = &self.cache {
            if *cache_drain == drain {
                return (*output).clone();
            }
        }

        // Build cache.
        let postclip_node = self.postclip_factory.generate(drain.clone());

        let resample_node = self.resample_factory.generate(postclip_node);

        let preclip_node = self.preclip_factory.generate(resample_node);

        let rotate_node = self.rotate_factory.generate(preclip_node);

        let out = self.transform_radians_factory.generate(rotate_node);

        // Populate cache.
        self.cache = Some((drain, out.clone()));

        // Output stage is a transform_radians node.
        out
    }
}

impl<'a, DRAIN, LINE, PR, PV, T> Transform for Projection<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
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
