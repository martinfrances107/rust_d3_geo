use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::compose::ComposeElemEnum;
use crate::Transform;

use super::rotation_identity::RotationIdentity;
use super::rotation_lambda::RotationLambda;
use super::rotation_phi_gamma::RotationPhiGamma;

#[derive(Clone)]
pub enum RotateRadiansEnum<T>
where
    T: CoordFloat + Default + FloatConst,
{
    C(Box<Compose<T>>),
    RL(RotationLambda<T>),
    RPG(RotationPhiGamma<T>),
    I(RotationIdentity<T>),
}

#[cfg(not(tarpaulin_include))]
impl<T> Debug for RotateRadiansEnum<T>
where
    T: CoordFloat + Default + FloatConst,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RotateRadiansEnum::C(_c) => {
                todo!("must find a way to do Box");
                // f.debug_struct("RotateRadiansEnum::C")
                // .field("0", "TODO")
                // .finish()
            }
            RotateRadiansEnum::RL(rl) => f
                .debug_struct("RotateRadiansEnum::RL")
                .field("0", rl)
                .finish(),
            RotateRadiansEnum::RPG(rpg) => f
                .debug_struct("RotateRadiansEnum::RPG")
                .field("0", rpg)
                .finish(),
            RotateRadiansEnum::I(i) => f
                .debug_struct("RotateRadiansEnum::I")
                .field("0", i)
                .finish(),
        }
    }
}

impl<T> Transform for RotateRadiansEnum<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type TcC = Coordinate<T>;
    fn transform(&self, p: &Self::TcC) -> Self::TcC {
        match self {
            RotateRadiansEnum::C(c) => c.transform(p),
            RotateRadiansEnum::RL(rl) => rl.transform(p),
            RotateRadiansEnum::RPG(rpg) => rpg.transform(p),
            RotateRadiansEnum::I(i) => i.transform(p),
        }
    }

    fn invert(&self, p: &Self::TcC) -> Self::TcC {
        match self {
            RotateRadiansEnum::C(c) => c.invert(p),
            RotateRadiansEnum::RL(rl) => rl.invert(p),
            RotateRadiansEnum::RPG(rpg) => rpg.invert(p),
            RotateRadiansEnum::I(i) => i.invert(p),
        }
    }
}

pub fn rotate_radians_transform<T: CoordFloat + Default + FloatConst>(
    delta_lambda_p: T,
    delta_phi: T,
    delta_gamma: T,
) -> RotateRadiansEnum<T> {
    let delta_lambda = delta_lambda_p % T::TAU();
    // Should I rotate by lambda, phi or gamma.
    let by_lambda = !delta_lambda.is_zero();
    let by_phi = !delta_phi.is_zero();
    let by_gamma = !delta_gamma.is_zero();
    return match (by_lambda, by_gamma, by_phi) {
        (true, true, true) | (true, true, false) | (true, false, true) => {
            RotateRadiansEnum::C(Box::new(Compose::new(
                ComposeElemEnum::RL(RotationLambda::new(delta_lambda)),
                ComposeElemEnum::RPG(RotationPhiGamma::new(&delta_phi, &delta_gamma)),
            )))
        }
        (true, false, false) => RotateRadiansEnum::RL(RotationLambda::new(delta_lambda)),
        (false, true, true) | (false, true, false) | (false, false, true) => {
            RotateRadiansEnum::RPG(RotationPhiGamma::new(&delta_phi, &delta_gamma))
        }
        (false, false, false) => RotateRadiansEnum::I(RotationIdentity::default()),
    };
}
