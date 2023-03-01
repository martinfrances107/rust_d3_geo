use std::marker::PhantomData;

use geo::CoordFloat;

use crate::projection::builder::Builder as BuilderCommon;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Build;
use crate::projection::Projector;
use crate::stream::Stream;
use crate::stream::Unconnected;

use super::Builder;
use super::PRConic;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> Build
    for Builder<BuilderCommon<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>, PR, T>
where
    BuilderCommon<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>: Clone
        + Build<
            ClipC = CLIPC,
            ClipU = CLIPU,
            Drain = DRAIN,
            PCNU = PCNU,
            PR = PR,
            RC = RC,
            RU = RU,
            T = T,
        >,
    DRAIN: Clone + Default + Stream<T = T>,
    CLIPC: Clone,
    CLIPU: Clone,
    PR: Clone + PRConic,
    PCNU: Clone,
    RU: Clone,
    T: CoordFloat,
{
    type ClipC = CLIPC;
    type ClipU = CLIPU;
    type Drain = DRAIN;
    type PCNU = PCNU;
    type PR = PR;
    type RC = RC;
    type RU = RU;
    type T = T;

    /// Using the currently programmed state output a new projection.
    #[inline]
    fn build(&self) -> Projector<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> {
        Projector {
            p_rc: PhantomData::<RC>,
            cache: None,
            postclip: self.base.postclip.clone(),
            clip: self.base.clip.clone(),
            resample: self.base.resample.clone(),
            rotator: self.base.rotator.clone(),
            project_rotate_transform: self.base.project_rotate_transform.clone(),
            transform_radians: StreamTransformRadians(Unconnected),
        }
    }
}
