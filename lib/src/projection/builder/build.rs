use geo::CoordFloat;

use crate::projection::builder::Builder;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Build;
use crate::projection::Projector;
use crate::stream::Unconnected;

impl<DRAIN, I, LC, LB, LU, PCNU, PR, PV, RC, RU, T> Build
    for Builder<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
    I: Clone,
    LC: Clone,
    LU: Clone,
    PCNU: Clone,
    PV: Clone,
    PR: Clone,
    RC: Clone,
    RU: Clone,
    T: CoordFloat,
{
    type Drain = DRAIN;
    type I = I;
    type LB = LB;
    type LC = LC;
    type LU = LU;
    type PCNU = PCNU;
    type PR = PR;
    type PV = PV;
    type RC = RC;
    type RU = RU;
    type T = T;
    /// Using the currently programmed state output a new projection.
    #[inline]
    fn build(&self) -> Projector<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T> {
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
