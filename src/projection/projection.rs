use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::clip::InterpolateRaw;
use crate::clip::LineRaw;
use crate::clip::PointVisible;
use crate::projection::resample::ResampleEnum;
use crate::projection::stream_node_factory::StreamNodeFactory;
use crate::projection::NodeFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::projection::StreamNode;
use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

// use derivative::Derivative;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::rotation::rotate_radians_enum::RotateRadiansEnum;
use crate::stream::Stream;
use crate::Transform;

use super::scale_translate_rotate::ScaleTranslateRotateEnum;
use super::stream_transform_radians::StreamTransformRadians;

// pub enum StreamOrValueMaybe<T: CoordFloat> {
//     Value(T),
//     SP(Box<dyn Stream<SC = Coordinate<T>>>),
// }

// #[derive(Derivative)]
// #[derivative(Debug)]
/// Projection
///
/// CF: ClipFactory
/// RF: ResampleFactory

// Projection<StreamNodeClipFactory<I, L, PV, T>, PR, StreamNodeFactory<R, T>, T>;
#[derive(Clone, Debug)]
pub struct Projection<DRAIN, I, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    I: InterpolateRaw,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    // pd: PhantomData<&'a u8>,
    projection_raw: PR,
    rotate: RotateRadiansEnum<T>, //rotate, pre-rotate

    /// Used exclusive by Transform( not stream releated).
    pub project_rotate_transform:
        Compose<T, RotateRadiansEnum<T>, Compose<T, PR, ScaleTranslateRotateEnum<T>>>,

    pub transform_radians_factory: StreamNodeFactory<StreamTransformRadians, DRAIN, T>,

    // project_transform_factory:
    //     StreamNodeFactory<Compose<T, PR, ScaleTranslateRotateEnum<T>>, DRAIN, T>,
    pub transform_rotate_factory: StreamNodeFactory<RotateRadiansEnum<T>, DRAIN, T>,
    preclip_factory: StreamNodeClipFactory<I, L, PV, DRAIN, T>,
    resample_factory: StreamNodeFactory<ResampleEnum<PR, T>, DRAIN, T>,

    // #[derivative(Debug = "ignore")]
    // postclip: fn(Box<DRAIN>) -> Box<DRAIN>,
    postclip: fn(Rc<RefCell<DRAIN>>) -> Rc<RefCell<DRAIN>>,
}

impl<'a, DRAIN, I, L, PR, PV, T> Projection<DRAIN, I, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    I: InterpolateRaw,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible,
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
    fn stream(
        &self,
        drain: Rc<RefCell<DRAIN>>,
    ) -> Rc<RefCell<StreamNode<StreamTransformRadians, DRAIN, T>>>
// where
    //     SD: Stream<SC = Coordinate<T>>,
    {
        // return cache && cacheStream === stream ? cache : cache = transformRadians(transformRotate(rotate)(preclip(projectResample(postclip(cacheStream = stream)))));
        // return match &self.cache {
        //     Some(c) => Box::new(*c),
        //     None => {
        // self.cache_stream = Some(stream.clone());

        let postclip = (self.postclip)(drain);

        // let mut resample = self.project_resample;
        let resample_node = self.resample_factory.generate(postclip);

        let preclip_node = self.preclip_factory.generate(resample_node);

        // using resample here bypasses preclip.
        // let t_rotate_node = StreamTransform::new(&self.rotate, self.preclip);
        // let t_rotate_node = StreamTransform::new(&self.rotate, self.project_resample);
        let transform_rotate_node = self.transform_rotate_factory.generate(preclip_node);

        let transform_radians_node = self
            .transform_radians_factory
            .generate(transform_rotate_node);
        // StreamNodeFactory::new(StreamTransformRadians::default()).generate(t_rotate_node);
        // t_radians_node.stream_in(t_rotate_node);

        // Output.
        transform_radians_node
    }
}

impl<'a, DRAIN, I, L, PR, PV, T> Transform for Projection<DRAIN, I, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    I: InterpolateRaw,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let r = Coordinate {
            x: p.x.to_radians(),
            y: p.y.to_radians(),
        };
        self.project_rotate_transform.transform(&r)
    }
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let d = self.project_rotate_transform.invert(p);
        Coordinate {
            x: d.x.to_degrees(),
            y: d.y.to_degrees(),
        }
    }
}
