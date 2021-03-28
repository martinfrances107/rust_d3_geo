use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::projection::scale_translate_rotate::ScaleTranslateRotateEnum;
use crate::projection::ProjectionRawEnum;
use crate::rotation::rotate_radians_transform::RotateRadiansEnum;
use crate::rotation::rotation_lambda::RotationLambda;
use crate::rotation::rotation_phi_gamma::RotationPhiGamma;
use crate::Transform;

#[derive(Clone, Debug)]
pub enum ComposeElemEnum<T>
where
    T: CoordFloat + FloatConst + Default,
{
    PRE(ProjectionRawEnum<T>),
    STR(ScaleTranslateRotateEnum<T>),
    RR(RotateRadiansEnum<T>),
    C(Box<Compose<T>>),
    RL(RotationLambda<T>),
    RPG(RotationPhiGamma<T>),
}

impl<T: AddAssign + CoordFloat + FloatConst + Default> Transform for ComposeElemEnum<T> {
    type TcC = Coordinate<T>;
    fn transform(&self, p: &Self::TcC) -> Self::TcC {
        match self {
            ComposeElemEnum::PRE(pre) => pre.transform(p),
            ComposeElemEnum::STR(str) => str.transform(p),
            ComposeElemEnum::RR(rr) => rr.transform(p),
            ComposeElemEnum::C(c) => c.transform(p),
            ComposeElemEnum::RL(rl) => rl.transform(p),
            ComposeElemEnum::RPG(rpg) => rpg.transform(p),
        }
    }
    fn invert(&self, p: &Self::TcC) -> Self::TcC {
        match self {
            ComposeElemEnum::PRE(pre) => pre.invert(p),
            ComposeElemEnum::STR(str) => str.invert(p),
            ComposeElemEnum::RR(rr) => rr.invert(p),
            ComposeElemEnum::C(c) => c.invert(p),
            ComposeElemEnum::RL(rl) => rl.invert(p),
            ComposeElemEnum::RPG(rpg) => rpg.invert(p),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Compose<T>
where
    T: CoordFloat + FloatConst + Default,
{
    pub a: ComposeElemEnum<T>,
    pub b: ComposeElemEnum<T>,
}

impl<T> Default for Compose<T>
where
    T: CoordFloat + FloatConst + Default,
{
    fn default() -> Self {
        Self {
            a: ComposeElemEnum::RL(RotationLambda::default()),
            b: ComposeElemEnum::RL(RotationLambda::default()),
        }
    }
}

impl<T: CoordFloat + FloatConst + Default> Compose<T> {
    #[inline]
    pub fn new(a: ComposeElemEnum<T>, b: ComposeElemEnum<T>) -> Compose<T> {
        Compose::<T> { a: a, b: b }
    }
}

impl<T: AddAssign + CoordFloat + FloatConst + Default> Transform for Compose<T> {
    type TcC = Coordinate<T>;
    // Apply A then B.
    fn transform(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.a.transform(coordinates);
        self.b.transform(&temp)
    }

    // Apply B them A.
    fn invert(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.b.invert(coordinates);
        self.a.invert(&temp)
    }
}
