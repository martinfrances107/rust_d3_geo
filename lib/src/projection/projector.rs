use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::clip::Clip;
use crate::clip::clip::Connected as ConnectedClip;
use crate::clip::Bufferable;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
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

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    DRAIN: Clone,
    I: Clone,
    LB: Clone,
    LC: Clone,
    LU: Clone,
    PR: Clone,
    PV: Clone,
    RC: Clone,
    RU: Clone,
    T: CoordFloat + FloatConst,
{
    /// Must be public as there is a implicit copy.
    pub p_pcnc: PhantomData<PCNC>,
    pub p_lb: PhantomData<LB>,
    pub p_lc: PhantomData<LC>,
    pub(crate) postclip: PCNU,

    pub(crate) resample: RU,

    pub(crate) clip: Clip<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, Unconnected, T>,

    pub(crate) rotator: RotatorRadians<Unconnected, T>,

    /// Used exclusively by Transform( not stream releated).
    // pub rotate_transform: Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,
    pub project_rotate_transform:
        Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,

    pub(crate) transform_radians: StreamTransformRadians<Unconnected>,
    pub(crate) cache: Option<(
        DRAIN,
        StreamTransformRadians<
            Connected<
                RotatorRadians<
                    Connected<
                        Clip<
                            DRAIN,
                            I,
                            LB,
                            LC,
                            LU,
                            PR,
                            PV,
                            RC,
                            RU,
                            ConnectedClip<DRAIN, LB, LC, LU, RC, RU, T>,
                            T,
                        >,
                    >,
                    T,
                >,
            >,
        >,
    )>,
}

impl<'a, DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Projector<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    // DRAIN: Stream<EP = DRAIN, T = T> + Default + PartialEq,
    DRAIN: Clone + PartialEq + Stream<EP = DRAIN, T = T>,
    I: Clone + Interpolator<EP = DRAIN, Stream = RC, T = T>,
    LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: Clone + LineConnected<SC = RC> + Stream<EP = DRAIN, T = T>,
    LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T>,
    PCNU: Clone + Connectable<SC = DRAIN, Output = PCNC>,
    PR: Clone,
    PV: Clone + PointVisible<T = T>,
    RU: Clone + Connectable<SC = PCNC, Output = RC>,
    RC: Clone + Stream<EP = DRAIN, T = T>,
    T: AsPrimitive<T> + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// Connects a DRAIN to the projection.
    ///
    /// The Projection Stream Pipeline :-
    ///
    /// StreamTransformRadians -> StreamTransform -> preclip -> resample -> postclip -> DRAIN
    ///
    pub fn stream(
        &mut self,
        drain: DRAIN,
    ) -> StreamTransformRadians<
        Connected<
            RotatorRadians<
                Connected<
                    Clip<
                        DRAIN,
                        I,
                        LB,
                        LC,
                        LU,
                        PR,
                        PV,
                        RC,
                        RU,
                        ConnectedClip<DRAIN, LB, LC, LU, RC, RU, T>,
                        T,
                    >,
                >,
                T,
            >,
        >,
    > {
        if let Some((cache_drain, output)) = &self.cache {
            if *cache_drain == drain {
                return (*output).clone();
            }
        }
        // Build cache.
        let postclip_node: PCNC = self.postclip.clone().connect(drain.clone());

        let resample_node: RC = self.resample.clone().connect(postclip_node);

        let preclip_node = self.clip.clone().connect(resample_node);

        let rotate_node = self.rotator.clone().connect(preclip_node);

        let out = self
            .transform_radians
            .clone()
            .connect::<DRAIN, _, T>(rotate_node);

        // Populate cache.
        self.cache = Some((drain, out.clone()));

        // Output stage is a transform_radians node.
        out
    }
}

impl<'a, DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Transform
    for Projector<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    DRAIN: Clone,
    I: Clone,
    LB: Clone,
    LC: Clone,
    LU: Clone,
    PR: Clone + Transform<T = T>,
    PV: Clone,
    RC: Clone,
    RU: Clone,
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
