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
use crate::Transform;

use super::rotation_identity::RotationIdentity;
use super::rotation_lambda::RotationLambda;
use super::rotation_phi_gamma::RotationPhiGamma;

pub enum RotateRadiansEnum<T>
where
    T: CoordFloat + Default + FloatConst,
{
    C(Box<Compose<T, RotationLambda<T>, RotationPhiGamma<T>>>),
    RL(RotationLambda<T>),
    RPG(RotationPhiGamma<T>),
    I(RotationIdentity<T>),
    Blank,
}

impl<T> Default for RotateRadiansEnum<T>
where
    T: CoordFloat + Default + FloatConst,
{
    fn default() -> Self {
        RotateRadiansEnum::Blank
    }
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
            RotateRadiansEnum::Blank => f
                .debug_struct("RotateRadiansEnum::I")
                .field("0", &String::from("RotateRadiansBlank"))
                .finish(),
        }
    }
}

impl<T> Transform for RotateRadiansEnum<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            RotateRadiansEnum::C(c) => c.transform(p),
            RotateRadiansEnum::RL(rl) => rl.transform(p),
            RotateRadiansEnum::RPG(rpg) => rpg.transform(p),
            RotateRadiansEnum::I(i) => i.transform(p),
            RotateRadiansEnum::Blank => {
                panic!("Calling transoform on a Blank")
            }
        }
    }

    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            RotateRadiansEnum::C(c) => c.invert(p),
            RotateRadiansEnum::RL(rl) => rl.invert(p),
            RotateRadiansEnum::RPG(rpg) => rpg.invert(p),
            RotateRadiansEnum::I(i) => i.invert(p),
            RotateRadiansEnum::Blank => {
                panic!("Calling transoform on a Blank")
            }
        }
    }
}
