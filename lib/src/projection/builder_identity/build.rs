use std::marker::PhantomData;

use geo::CoordFloat;

use super::Builder;
use crate::projection::projector_identity::transformer::Transformer;
use crate::projection::projector_identity::Projector;

impl<DRAIN, PCNU, T> Builder<DRAIN, PCNU, T>
where
    PCNU: Clone,
    T: CoordFloat,
{
    #[inline]
    pub fn build<PCNC: Clone>(&self) -> Projector<DRAIN, PCNC, PCNU, T> {
        Projector {
            p_pcnc: PhantomData::<PCNC>,
            transform: Transformer::new(
                self.alpha, self.kx, self.ky, self.ca, self.sa, self.tx, self.ty,
            ),
            postclip: self.postclip.clone(),
            cache: None,
        }
    }
}
