use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::Builder as BuilderCommon;
use crate::projection::projector_commom::Projector;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Build;

use crate::stream::Unconnected;

use super::Builder;
use super::PRConic;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T> Build
    for Builder<BuilderCommon<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>, T>
where
    BuilderCommon<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>:
        Clone + Build<Projector = Projector<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>>,
    DRAIN: Clone,
    CLIPC: Clone,
    CLIPU: Clone,
    PR: Clone + PRConic,
    PCNU: Clone,
    RU: Clone,
    T: CoordFloat + FloatConst,
{
    type Projector = Projector<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>;

    /// Using the currently programmed state output a new projection.
    #[inline]
    fn build(&self) -> Self::Projector {
        Self::Projector {
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
