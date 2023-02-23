use std::marker::PhantomData;

use geo::CoordFloat;

use crate::projection::builder::Builder as BuilderCommon;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Projector;
use crate::stream::Unconnected;

use super::Builder;
use super::PRConic;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
    Builder<BuilderCommon<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>, PR, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    PR: Clone + PRConic,
    PCNU: Clone,
    RU: Clone,
    T: CoordFloat,
{
    // type Projector<CLIPC, CLIPU, DRAIN, PCNU, RC, RU> =
    //     Projector<CLIPC, CLIPU, DRAIN, PCNU, PRConic, RC, RU, T>;
    /// Using the currently programmed state output a new projection.
    #[inline]
    pub fn build(&self) -> Projector<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> {
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
