use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable as ClipConnectable;
use crate::compose::Compose;
use crate::rot::rotate_radians::RotateRadians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::stream_transform_radians::StreamTransformRadians;
use super::transform::scale_translate_rotate::ScaleTranslateRotate;

/// Builder shorthand notations.
pub mod types;

type CacheState<CLIP, DRAIN, T> = Option<(
    DRAIN,
    StreamTransformRadians<Connected<RotatorRadians<Connected<CLIP>, T>>>,
)>;

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    pub(crate) p_rc: PhantomData<RC>,
    /// Must be public as there is a implicit copy.
    pub(crate) postclip: PCNU,

    pub(crate) resample: RU,

    pub(crate) clip: CLIPU,

    pub(crate) rotator: RotatorRadians<Unconnected, T>,

    /// Transform applied after conversion to radians.
    pub project_rotate_transform:
        Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,

    pub(crate) transform_radians: StreamTransformRadians<Unconnected>,
    pub(crate) cache: CacheState<CLIPC, DRAIN, T>,
}

type ProjectorStream<CLIP, T> =
    StreamTransformRadians<Connected<RotatorRadians<Connected<CLIP>, T>>>;

impl<CC, CU, DRAIN, PCNC, PCNU, PR, RC, RU, T> Projector<CC, CU, DRAIN, PCNU, PR, RC, RU, T>
where
    CC: Clone + Stream<EP = DRAIN, T = T>,
    CU: Clone + ClipConnectable<Output = CC, SC = RC>,
    DRAIN: Clone + PartialEq,
    PCNC: Clone,
    PCNU: Clone + Connectable<Output<DRAIN> = PCNC>,
    RU: Clone + Connectable<Output<PCNC> = RC>,
    RC: Clone,
    T: CoordFloat,
{
    /// Connects a DRAIN to the projection.
    ///
    /// The Projection Stream Pipeline :-
    ///
    /// `StreamTransformRadians` -> `StreamTransform` -> `Preclip` -> `Resample` -> `Postclip` -> `DRAIN`
    ///
    pub fn stream(&mut self, drain: &DRAIN) -> ProjectorStream<CC, T> {
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

        let out = self.transform_radians.clone().connect::<_>(rotate_node);

        // Populate cache.
        self.cache = Some((drain.clone(), out.clone()));

        // Output stage is a transform_radians node.
        out
    }
}

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> Transform
    for Projector<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,

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
