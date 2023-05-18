use geo::CoordFloat;

use crate::projection::projector_commom::Projector;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Build;

use crate::stream::Unconnected;

use super::Builder;

impl<CLIPC, CLIPU, PCNU, PR, RU, T> Build for Builder<CLIPC, CLIPU, PCNU, PR, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    PCNU: Clone,
    PR: Clone,
    RU: Clone,
    T: CoordFloat,
{
    type Projector<DRAIN> = Projector<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>;
    /// Using the currently programmed state output a new projection.
    #[inline]
    fn build<DRAIN>(&self) -> Self::Projector<DRAIN> {
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
