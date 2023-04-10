use geo::CoordFloat;

use crate::projection::projector_commom::Projector;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Build;

use crate::stream::Unconnected;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T> Build for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    PCNU: Clone,
    PR: Clone,
    RU: Clone,
    T: CoordFloat,
{
    type Projector = Projector<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>;
    /// Using the currently programmed state output a new projection.
    #[inline]
    fn build(&self) -> Self::Projector {
        Projector {
            cache: None,
            postclip: self.postclip.clone(),
            clip: self.clip.clone(),
            resample: self.resample.clone(),
            rotator: self.rotator.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            transform_radians: StreamTransformRadians(Unconnected),
        }
    }
}
