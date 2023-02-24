use geo::CoordFloat;

use super::Builder;
use super::PRConic;
use super::ParallelsGet;
use crate::projection::BuilderTrait;

// Reach into builder and alter the PR.
impl<BASE, PR, T> ParallelsGet for Builder<BASE, PR, T>
where
    BASE: BuilderTrait<PR = PR>,
    PR: PRConic<T = T> + Clone,
    T: CoordFloat,
{
    type T = T;

    fn parallels(&mut self) -> (Self::T, Self::T) {
        (self.phi0.to_degrees(), self.phi1.to_degrees())
    }
}
