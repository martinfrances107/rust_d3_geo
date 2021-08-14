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
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;
use crate::Transform;

use super::rotation_identity::RotationIdentity;
use super::rotation_lambda::RotationLambda;
use super::rotation_phi_gamma::RotationPhiGamma;

pub enum RotateRadiansEnum<T>
where
    T: CoordFloat + FloatConst,
{
    C(Box<Compose<T, RotationLambda<T>, RotationPhiGamma<T>>>),
    RL(RotationLambda<T>),
    RPG(RotationPhiGamma<T>),
    I(RotationIdentity<T>),
}

impl<T> Default for RotateRadiansEnum<T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        RotateRadiansEnum::I(RotationIdentity::default())
    }
}

impl<T> Clone for RotateRadiansEnum<T>
where
    T: CoordFloat + FloatConst,
{
    fn clone(&self) -> Self {
        match self {
            RotateRadiansEnum::C(c) => RotateRadiansEnum::C(Box::new(*c.clone())),
            RotateRadiansEnum::RL(rl) => RotateRadiansEnum::RL(rl.clone()),
            RotateRadiansEnum::RPG(rpg) => RotateRadiansEnum::RPG(rpg.clone()),
            RotateRadiansEnum::I(i) => RotateRadiansEnum::I(i.clone()),
        }
    }
}
#[cfg(not(tarpaulin_include))]
impl<T> Debug for RotateRadiansEnum<T>
where
    T: CoordFloat + FloatConst,
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
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            RotateRadiansEnum::C(c) => c.transform(p),
            RotateRadiansEnum::RL(rl) => rl.transform(p),
            RotateRadiansEnum::RPG(rpg) => rpg.transform(p),
            RotateRadiansEnum::I(i) => i.transform(p),
        }
    }

    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            RotateRadiansEnum::C(c) => c.invert(p),
            RotateRadiansEnum::RL(rl) => rl.invert(p),
            RotateRadiansEnum::RPG(rpg) => rpg.invert(p),
            RotateRadiansEnum::I(i) => i.invert(p),
        }
    }
}

impl<SINK, T> Stream for StreamNode<RotateRadiansEnum<T>, SINK, T>
where
    SINK: Stream<SC = Coordinate<T>>,
    T: CoordFloat + FloatConst,
{
    type SC = Coordinate<T>;
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        self.sink.borrow_mut().point(&self.raw.transform(p), m);
    }
    #[inline]
    fn sphere(&mut self) {
        self.sink.borrow_mut().sphere();
    }
    #[inline]
    fn line_start(&mut self) {
        self.sink.borrow_mut().line_start();
    }
    #[inline]
    fn line_end(&mut self) {
        self.sink.borrow_mut().line_end();
    }
    #[inline]
    fn polygon_start(&mut self) {
        self.sink.borrow_mut().polygon_start()
    }
    #[inline]
    fn polygon_end(&mut self) {
        self.sink.borrow_mut().polygon_end();
    }
}
