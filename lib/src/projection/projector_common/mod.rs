use core::fmt::Debug;
use std::sync::mpsc;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::RecvError;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::compose::Compose;
use crate::projection::Projector as ProjectorTrait;
use crate::rot::rotate_radians::RotateRadians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::EndPointMT;
use crate::stream::StreamMT;
use crate::stream::Unconnected;
use crate::Transform;

use super::stream_transform_radians::StreamTransformRadians;
use super::transform::scale_translate_rotate::ScaleTranslateRotate;

/// Builder shorthand notations.
pub mod types;

type CacheState<DRAIN, SOURCE> = Option<(DRAIN, SOURCE)>;

/// Projection output of projection/Builder.
///
/// Common functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<CLIPU, DRAIN, PCNU, PR, RU, SOURCE, T>
where
    T: CoordFloat,
{
    /// Must be public as there is a implicit copy.
    pub(crate) postclip: PCNU,

    pub(crate) resample: RU,

    pub(crate) clip: CLIPU,

    pub(crate) rotator: RotatorRadians<Unconnected, T>,

    /// Transform applied after conversion to radians.
    pub project_rotate_transform:
        Compose<RotateRadians<T>, Compose<PR, ScaleTranslateRotate<T>>>,

    pub(crate) transform_radians: StreamTransformRadians<Unconnected, T>,
    pub(crate) cache: CacheState<DRAIN, SOURCE>,
}

/// The entry point on the path
///
///  A connected version of the ``StreamTransformRadians`` transformer
pub type Source<CLIPC, T> = StreamTransformRadians<Connected<Rrc<CLIPC, T>>, T>;

/// A connection version of the ``RotateRadians`` transformer
pub(super) type Rrc<CLIPC, T> = RotatorRadians<Connected<CLIPC>, T>;

impl<CLIPC, CLIPU, DRAIN, PCNC, PCNU, PR, RC, RU, T> ProjectorTrait
    for Projector<CLIPU, DRAIN, PCNU, PR, RU, Source<CLIPC, T>, T>
where
    CLIPC: Clone,
    CLIPU: ConnectableClip<Output = CLIPC, SC = RC>,
    DRAIN: Clone + PartialEq,
    PCNU: Clone + Connectable<Output<DRAIN> = PCNC>,
    PR: Transform<T = T>,
    RU: Clone + Connectable<Output<PCNC> = RC>,

    T: CoordFloat + FloatConst,
{
    /// Connects a DRAIN to the projection.
    ///
    /// The Projection Stream Path :-
    ///
    /// `StreamTransformRadians` -> `StreamTransform` -> `Preclip` -> `Resample` -> `Postclip` -> `DRAIN`
    ///

    type EP = DRAIN;

    type Transformer = Source<CLIPC, T>;

    fn stream(&mut self, drain: &DRAIN) -> Self::Transformer {
        if let Some((cache_drain, output)) = &self.cache {
            if *cache_drain == *drain {
                return (*output).clone();
            }
        }
        // Build cache.
        let postclip_node = self.postclip.clone().connect(drain.clone());

        let resample_node = self.resample.clone().connect(postclip_node);

        let preclip_node = self.clip.connect(resample_node);

        let rotate_node = self.rotator.clone().connect(preclip_node);

        let out = self.transform_radians.clone().connect(rotate_node);

        // Populate cache.
        self.cache = Some((drain.clone(), out.clone()));

        // First stage is a transform radians node.
        out
    }
}

/// Multi-thread Stream: All messages
#[derive(Debug)]
pub enum Message<T>
where
    T: CoordFloat,
{
    /// Returns the end point of the stream.
    EndPoint(EndPointMT<T>),
    /// Declare the end of a line segment.
    LineEnd,
    /// Declare the start of a line segment.
    LineStart,
    /// Declare the start of a polygon.
    PolygonStart,
    /// Declare a point.
    ///
    /// TODO can I pass Coord by reference
    /// * make this Sync -
    /// * resolve lifetime issues.
    Point((Coord<T>, Option<u8>)),
    /// Declare the end of a polygon.
    PolygonEnd,
    /// Declare a sphere object.
    Sphere,
    /// Termintate threads, close all channels
    ShutDown,
    /// Send result while shutting down.
    ShutDownWithReturn(EndPointMT<T>),
}

/// Multi-thread Streams
///
/// State of the thread/stage upon termination.
#[derive(Debug)]
pub enum ChannelStatus<T>
where
    T: CoordFloat,
{
    /// The channel sink collapsed.
    Rx(RecvError),
    /// The entrance to the channel collapsed.
    Tx(SendError<Message<T>>),
    /// Message is terminting without error.
    ShuntDownReceived,
}

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
    Projector<CLIPU, DRAIN, PCNU, PR, RU, Source<CLIPC, T>, T>
where
    CLIPC: Clone,
    CLIPU: ConnectableClip<Output = CLIPC, SC = RC>,
    DRAIN: Clone + PartialEq,
    PCNU: Clone + StreamMT<T>,
    PR: Transform<T = T>,
    RU: Clone + StreamMT<T>,
    T: 'static + CoordFloat + FloatConst + Send,
{
    fn stream_mt(&self, drain: &DRAIN) -> Vec<JoinHandle<ChannelStatus<T>>> {
        // Prepare stage-interlink channels
        // Input to stage txN. rxN consumed in stage N.
        let (tx1, rx1) = sync_channel(100);

        let (tx2, rx2) = sync_channel(100);

        let (tx3, rx3) = sync_channel(100);
        // let (tx4, rx4): (Sender<Message<T>>, Receiver<Message<T>>) =
        //     mpsc::channel();
        // let (tx5, rx5): (Sender<Message<T>>, Receiver<Message<T>>) =
        //     mpsc::channel();

        let mut handles = vec![];

        // Build cache.
        let postclip_node = self.postclip.clone();

        let resample_node = self.resample.clone();

        // let preclip_node = self.clip;

        let rotate_node = self.rotator.clone();

        let out: StreamTransformRadians<_, T> = self.transform_radians.clone();

        let stage1 = out.gen_stage(tx1, rx1);
        handles.push(stage1);
        let stage2 = rotate_node.gen_stage(tx2, rx2);
        handles.push(stage2);
        let stage3 = resample_node.gen_stage(tx3, rx3);
        handles.push(stage3);

        // return thread bundle.
        handles
    }
}

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T> Transform
    for Projector<CLIPU, DRAIN, PCNU, PR, RU, Source<CLIPC, T>, T>
where
    PR: Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let r = Coord {
            x: p.x.to_radians(),
            y: p.y.to_radians(),
        };
        self.project_rotate_transform.transform(&r)
    }
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        let d = self.project_rotate_transform.invert(p);
        Coord {
            x: d.x.to_degrees(),
            y: d.y.to_degrees(),
        }
    }
}
