use geo::CoordFloat;

use crate::projection::RotateGet;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T> RotateGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn rotate(&self) -> [T; 3] {
        [
            self.delta_lambda.to_degrees(),
            self.delta_phi.to_degrees(),
            self.delta_gamma.to_degrees(),
        ]
    }
}
