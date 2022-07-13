use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::clip::Clip;
use crate::clip::clip::Connected as ConnectedClip;
use crate::clip::Bufferable;
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

pub mod types;

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
    T: CoordFloat,
{
    /// Must be public as there is a implicit copy.
    pub(crate) postclip: PCNU,

    pub(crate) resample: RU,

    pub(crate) clip: Clip<I, LC, LU, PV, RC, Unconnected, T>,

    pub(crate) rotator: RotatorRadians<Unconnected, T>,

    pub project_rotate_transform:
        Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,

    pub(crate) transform_radians: StreamTransformRadians<Unconnected>,
    pub(crate) cache: Option<(
        DRAIN,
        StreamTransformRadians<
            Connected<
                RotatorRadians<Connected<Clip<I, LC, LU, PV, RC, ConnectedClip<LB, LC, T>, T>>, T>,
            >,
        >,
    )>,
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Projector<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
    DRAIN: Clone + PartialEq + Stream<EP = DRAIN, T = T>,
    I: Clone,
    LB: Clone,
    LC: Clone,
    LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T>,
    PCNU: Clone + Connectable<SC = DRAIN, Output = PCNC>,
    PR: Clone,
    PV: Clone,
    RU: Clone + Connectable<SC = PCNC, Output = RC>,
    RC: Clone,
    PCNU: Clone,
    T: CoordFloat + FloatConst,
{
    /// Connects a DRAIN to the projection.
    ///
    /// The Projection Stream Pipeline :-
    ///
    /// StreamTransformRadians -> StreamTransform -> preclip -> resample -> postclip -> DRAIN
    ///
    pub fn stream(
        &mut self,
        drain: &DRAIN,
    ) -> StreamTransformRadians<
        Connected<
            RotatorRadians<Connected<Clip<I, LC, LU, PV, RC, ConnectedClip<LB, LC, T>, T>>, T>,
        >,
    > {
        if let Some((cache_drain, output)) = &self.cache {
            if *cache_drain == *drain {
                return (*output).clone();
            }
        }
        // Build cache.
        let postclip_node = self.postclip.clone().connect(drain.clone());

        let resample_node = self.resample.clone().connect(postclip_node);

        let preclip_node = self.clip.clone().connect(resample_node);

        let rotate_node = self.rotator.clone().connect(preclip_node);

        let out = self
            .transform_radians
            .clone()
            .connect::<DRAIN, _, T>(rotate_node);

        // Populate cache.
        self.cache = Some((drain.clone(), out.clone()));

        // Output stage is a transform_radians node.
        out
    }
}

impl<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T> Transform
    for Projector<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
    PR: Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

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
