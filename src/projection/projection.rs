use crate::clip::PostClipFn;
use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

// use derivative::Derivative;
use derivative::Derivative;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::clip::Clip;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::clip::LineRaw;
use crate::clip::PointVisible;
use crate::compose::Compose;
use crate::rotation::rotate_radians_enum::RotateRadiansEnum;
use crate::stream::Stream;
use crate::Transform;

use super::resample::ResampleEnum;
use super::scale_translate_rotate::ScaleTranslateRotateEnum;
use super::stream_node_factory::StreamNodeFactory;
use super::stream_transform_radians::StreamTransformRadians;
use super::NodeFactory;
use super::Raw as ProjectionRaw;
use super::StreamNode;

// pub enum StreamOrValueMaybe<T: CoordFloat> {
//     Value(T),
//     SP(Box<dyn Stream<SC = Coordinate<T>>>),
// }

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct Projection<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    #[derivative(Debug = "ignore")]
    pub postclip: PostClipFn<DRAIN>,
    pub projection_raw: PR,
    pub preclip_factory:
        StreamNodeClipFactory<L, PR, PV, StreamNode<ResampleEnum<PR, T>, DRAIN, T>, T>,

    pub resample_factory: StreamNodeFactory<ResampleEnum<PR, T>, DRAIN, T>,

    pub rotate: RotateRadiansEnum<T>, //rotate, pre-rotate

    /// Used exclusive by Transform( not stream releated).
    pub rotate_transform:
        Compose<T, RotateRadiansEnum<T>, Compose<T, PR, ScaleTranslateRotateEnum<T>>>,

    pub rotate_transform_factory: StreamNodeFactory<
        Compose<T, RotateRadiansEnum<T>, Compose<T, PR, ScaleTranslateRotateEnum<T>>>,
        StreamNode<
            Clip<L, PV, StreamNode<ResampleEnum<PR, T>, DRAIN, T>, T>,
            StreamNode<ResampleEnum<PR, T>, DRAIN, T>,
            T,
        >,
        T,
    >,

    pub transform_radians_factory: StreamNodeFactory<
        StreamTransformRadians,
        StreamNode<
            Compose<T, RotateRadiansEnum<T>, Compose<T, PR, ScaleTranslateRotateEnum<T>>>,
            StreamNode<
                Clip<L, PV, StreamNode<ResampleEnum<PR, T>, DRAIN, T>, T>,
                StreamNode<ResampleEnum<PR, T>, DRAIN, T>,
                T,
            >,
            T,
        >,
        T,
    >,
}

impl<'a, DRAIN, L, PR, PV, T> Projection<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
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
    pub fn stream(
        &self,
        drain: Rc<RefCell<DRAIN>>,
    ) -> StreamNode<
        StreamTransformRadians,
        StreamNode<
            Compose<T, RotateRadiansEnum<T>, Compose<T, PR, ScaleTranslateRotateEnum<T>>>,
            StreamNode<
                Clip<L, PV, StreamNode<ResampleEnum<PR, T>, DRAIN, T>, T>,
                StreamNode<ResampleEnum<PR, T>, DRAIN, T>,
                T,
            >,
            T,
        >,
        T,
    > {
        let postclip = (self.postclip)(drain);

        let resample_node = Rc::new(RefCell::new(self.resample_factory.generate(postclip)));

        let preclip_node = Rc::new(RefCell::new(self.preclip_factory.generate(resample_node)));

        let rotate_transform_node = Rc::new(RefCell::new(
            self.rotate_transform_factory.generate(preclip_node),
        ));

        // Output stage is a transform_radians node.
        self.transform_radians_factory
            .generate(rotate_transform_node)
    }
}

impl<'a, DRAIN, L, PR, PV, T> Transform for Projection<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type C = Coordinate<T>;
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
