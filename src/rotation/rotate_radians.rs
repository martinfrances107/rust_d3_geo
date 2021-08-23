use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;
use crate::Transform;

use super::rotation_identity::RotationIdentity;
use super::rotation_lambda::RotationLambda;
use super::rotation_phi_gamma::RotationPhiGamma;

pub enum RotateRadiams<T>
where
    T: CoordFloat + FloatConst,
{
    C(Box<Compose<T, RotationLambda<T>, RotationPhiGamma<T>>>),
    RL(RotationLambda<T>),
    RPG(RotationPhiGamma<T>),
    I(RotationIdentity<T>),
}

impl<T> Default for RotateRadiams<T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        RotateRadiams::I(RotationIdentity::default())
    }
}

impl<T> Clone for RotateRadiams<T>
where
    T: CoordFloat + FloatConst,
{
    fn clone(&self) -> Self {
        match self {
            RotateRadiams::C(c) => RotateRadiams::C(Box::new(*c.clone())),
            RotateRadiams::RL(rl) => RotateRadiams::RL(*rl),
            RotateRadiams::RPG(rpg) => RotateRadiams::RPG(*rpg),
            RotateRadiams::I(i) => RotateRadiams::I(*i),
        }
    }
}
#[cfg(not(tarpaulin_include))]
impl<T> Debug for RotateRadiams<T>
where
    T: CoordFloat + FloatConst,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RotateRadiams::C(_c) => {
                todo!("must find a way to do Box");
                // f.debug_struct("RotateRadiams::C")
                // .field("0", "TODO")
                // .finish()
            }
            RotateRadiams::RL(rl) => f.debug_struct("RotateRadiams::RL").field("0", rl).finish(),
            RotateRadiams::RPG(rpg) => f
                .debug_struct("RotateRadiams::RPG")
                .field("0", rpg)
                .finish(),
            RotateRadiams::I(i) => f.debug_struct("RotateRadiams::I").field("0", i).finish(),
        }
    }
}

impl<T> Transform for RotateRadiams<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            RotateRadiams::C(c) => c.transform(p),
            RotateRadiams::RL(rl) => rl.transform(p),
            RotateRadiams::RPG(rpg) => rpg.transform(p),
            RotateRadiams::I(i) => i.transform(p),
        }
    }

    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            RotateRadiams::C(c) => c.invert(p),
            RotateRadiams::RL(rl) => rl.invert(p),
            RotateRadiams::RPG(rpg) => rpg.invert(p),
            RotateRadiams::I(i) => i.invert(p),
        }
    }
}

impl<SINK, T> Stream for StreamNode<RotateRadiams<T>, SINK, T>
where
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;
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
