use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::projection::builder::Builder as BuilderCommon;
use crate::projection::projector_common::Projector;
use crate::projection::projector_common::Source;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Build;

use crate::stream::Unconnected;

use super::Builder;
use super::PRConic;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T> Build
    for Builder<BuilderCommon<CLIPU, DRAIN, PCNU, PR, RU, T>, T>
where
    CLIPU: Clone + ConnectableClip<Output = CLIPC>,
    PR: Clone + PRConic,
    PCNU: Clone,
    RU: Clone,
    T: CoordFloat + FloatConst,
{
    type Projector = Projector<CLIPU, DRAIN, PCNU, PR, RU, Source<CLIPC, T>, T>;

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
