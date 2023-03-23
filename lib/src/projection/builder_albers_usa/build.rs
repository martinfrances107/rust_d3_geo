use super::Builder;
use crate::projection::projector_albers_usa::Projector;

impl<DRAIN> Builder<DRAIN>
where
    DRAIN: Clone,
{
    /// Using the currently programmed state output a new projection.
    #[inline]
    pub fn build<MULTIPLEX>(&self) -> Projector<DRAIN, MULTIPLEX> {
        todo!();
        // Projector {
        //     multiplex: Multiplex::new([
        //         self.pr.alaska_point,
        //         self.pr.lower_48_point,
        //         self.pr.hawaii_point,
        //     ]),
        // }
    }
}
