use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

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
pub enum RotateRadians<T> {
    /// A combination of rotations.
    C(Box<Compose<RotationLambda<T>, RotationPhiGamma<T>>>),
    /// Just roation in one direction.
    RL(RotationLambda<T>),
    /// Rotate, Phi and Gamma
    RPG(RotationPhiGamma<T>),
    /// No rotation.
    I(RotationIdentity<T>),
}

#[cfg(not(tarpaulin_include))]
impl<T> Debug for RotateRadians<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::C(_c) => {
                // todo!("must find a way to do Box");
                f.debug_struct("RotateRadians::C").finish()
            }
            Self::RL(rl) => f.debug_struct("RotateRadians::RL").field("0", rl).finish(),
            Self::RPG(rpg) => f
                .debug_struct("RotateRadians::RPG")
                .field("0", rpg)
                .finish(),
            Self::I(i) => f.debug_struct("RotateRadians::I").field("0", i).finish(),
        }
    }
}

impl<T> Clone for RotateRadians<T>
where
    T: CoordFloat,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::C(c) => Self::C(Box::new(*c.clone())),
            Self::RL(rl) => Self::RL(rl.clone()),
            Self::RPG(rpg) => Self::RPG(rpg.clone()),
            Self::I(i) => Self::I(i.clone()),
        }
    }
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
