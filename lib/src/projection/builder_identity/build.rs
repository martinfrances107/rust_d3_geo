use geo::CoordFloat;

use super::Builder;
use crate::projection::projector_identity::transformer::Transformer;
use crate::projection::projector_identity::Projector;

impl<PCNU, T> Builder<PCNU, T>
where
    PCNU: Clone,
    T: CoordFloat,
{
    #[inline]
    /// Returns a projectors based on the builder settings.
    pub fn build<DRAIN, PCNC>(&self) -> Projector<DRAIN, PCNC, PCNU, T> {
        Projector {
            transform: Transformer::new(
                self.alpha, self.kx, self.ky, self.ca, self.sa, self.tx, self.ty,
            ),
            postclip: self.postclip.clone(),
            cache: None,
        }
    }
}
