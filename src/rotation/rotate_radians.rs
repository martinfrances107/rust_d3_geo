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

/// A 3-axis rotation transform.
pub enum RotateRadians<T>
where
    T: CoordFloat + FloatConst,
{
    /// A combination of rotations.
    C(Box<Compose<T, RotationLambda<T>, RotationPhiGamma<T>>>),
    /// Just roation in one direction.
    RL(RotationLambda<T>),
    /// Rotate, Phi and Gamma.
    RPG(RotationPhiGamma<T>),
    /// No rotation.
    I(RotationIdentity<T>),
}

impl<T> Default for RotateRadians<T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        RotateRadians::I(RotationIdentity::default())
    }
}

impl<T> Clone for RotateRadians<T>
where
    T: CoordFloat + FloatConst,
{
    fn clone(&self) -> Self {
        match self {
            RotateRadians::C(c) => RotateRadians::C(Box::new(*c.clone())),
            RotateRadians::RL(rl) => RotateRadians::RL(*rl),
            RotateRadians::RPG(rpg) => RotateRadians::RPG(*rpg),
            RotateRadians::I(i) => RotateRadians::I(*i),
        }
    }
}
#[cfg(not(tarpaulin_include))]
impl<T> Debug for RotateRadians<T>
where
    T: CoordFloat + FloatConst,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RotateRadians::C(_c) => {
                todo!("must find a way to do Box");
                // f.debug_struct("RotateRadians::C")
                // .field("0", "TODO")
                // .finish()
            }
            RotateRadians::RL(rl) => f.debug_struct("RotateRadians::RL").field("0", rl).finish(),
            RotateRadians::RPG(rpg) => f
                .debug_struct("RotateRadians::RPG")
                .field("0", rpg)
                .finish(),
            RotateRadians::I(i) => f.debug_struct("RotateRadians::I").field("0", i).finish(),
        }
    }
}

impl<T> Transform for RotateRadians<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            RotateRadians::C(c) => c.transform(p),
            RotateRadians::RL(rl) => rl.transform(p),
            RotateRadians::RPG(rpg) => rpg.transform(p),
            RotateRadians::I(i) => i.transform(p),
        }
    }

    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            RotateRadians::C(c) => c.invert(p),
            RotateRadians::RL(rl) => rl.invert(p),
            RotateRadians::RPG(rpg) => rpg.invert(p),
            RotateRadians::I(i) => i.invert(p),
        }
    }
}

impl<SINK, T> Stream for StreamNode<RotateRadians<T>, SINK, T>
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
