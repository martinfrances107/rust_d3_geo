use core::fmt::Debug;

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

    pub(crate) transform_radians: StreamTransformRadians<Unconnected>,
    pub(crate) cache: CacheState<DRAIN, SOURCE>,
}

/// The entry point on the path
///
///  A connected version of the ``StreamTransformRadians`` transformer
pub type Source<CLIPC, T> = StreamTransformRadians<Connected<Rrc<CLIPC, T>>>;

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
