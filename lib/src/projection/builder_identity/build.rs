use geo::CoordFloat;

use crate::projection::projector_identity::Projector;

use super::Builder;

impl<DRAIN, PCNU, T> Builder<DRAIN, PCNU, T>
where
    PCNU: Clone,
    T: CoordFloat,
{
    // type Drain = DRAIN;
    // type PCNU = PCNU;
    // type T = T;
    /// Using the currently programmed state output a new projection.
    #[inline]
    pub fn build<PCNC>(&self) -> Projector<DRAIN, PCNC, PCNU, T> {
        todo!();
        // Projector {
        //     cache: None,
        //     postclip: self.postclip.clone(),
        //     clip: self.clip.clone(),
        //     resample: self.resample.clone(),
        //     rotator: self.rotator.clone(),
        //     project_rotate_transform: self.project_rotate_transform.clone(),
        //     transform_radians: StreamTransformRadians(Unconnected),
        // }
    }
}
