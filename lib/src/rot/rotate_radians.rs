use core::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::Transform;

use super::rotation_identity::RotationIdentity;
use super::rotation_lambda::RotationLambda;
use super::rotation_phi_gamma::RotationPhiGamma;

/// Container for a 3-axis rotation transform.
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub enum RotateRadians<T> {
    /// A combination of rotations.
    C(Compose<RotationLambda<T>, RotationPhiGamma<T>>),
    /// Just rotation in one direction.
    RL(RotationLambda<T>),
    /// Rotate, Phi and Gamma
    RPG(RotationPhiGamma<T>),
    /// No rotation.
    I(RotationIdentity<T>),
}

impl<T> Transform for RotateRadians<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        match self {
            Self::C(c) => c.transform(p),
            Self::RL(rl) => rl.transform(p),
            Self::RPG(rpg) => rpg.transform(p),
            Self::I(i) => i.transform(p),
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        match self {
            Self::C(c) => c.invert(p),
            Self::RL(rl) => rl.invert(p),
            Self::RPG(rpg) => rpg.invert(p),
            Self::I(i) => i.invert(p),
        }
    }
}
