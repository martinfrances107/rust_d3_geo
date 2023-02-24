use geo::CoordFloat;

use super::Builder;
use super::PRConic;
use super::ParallelsSet;
use crate::projection::BuilderTrait;

// Reach into builder and alter the PR.
impl<BASE, PR, T> ParallelsSet for Builder<BASE, PR, T>
where
    BASE: BuilderTrait<PR = PR>,
    PR: PRConic<T = T> + Clone,
    T: CoordFloat,
{
    type T = T;

    fn parallels_set(&mut self, phi0: T, phi1: T) -> &mut Self {
        let projection_raw =
            <PR as PRConic>::generate(self.pr.clone(), phi0.to_radians(), phi1.to_radians());
        self.base.update_pr(projection_raw);
        self
    }
}
