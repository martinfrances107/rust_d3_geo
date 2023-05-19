use geo::CoordFloat;

use crate::projection::projector_commom::Projector;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Build;

use crate::stream::Unconnected;

use super::Builder;

impl<CLIPU, PCNU, PR, RU, T> Build for Builder<CLIPU, PCNU, PR, RU, T>
where
    CLIPU: Clone,
    PCNU: Clone,
    PR: Clone,
    RU: Clone,
    T: CoordFloat,
{
    type Projector<CLIPC, DRAIN> = Projector<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>;
    /// Using the currently programmed state output a new projection.
    #[inline]
    fn build<CLIPC, DRAIN>(&self) -> Self::Projector<CLIPC, DRAIN> {
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
