use std::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::rot::rotate_radians::RotateRadians;
use crate::rot::rotation_identity::RotationIdentity;
use crate::stream::Stream as StreamTrait;
use crate::Transform;

/// Output of `CircleGenertor::circle()`.
#[derive(Clone, Debug)]
pub struct Stream<T>
where
    T: CoordFloat,
{
    /// The rotation used to generate the circle stream.
    pub rotate: RotateRadians<T>,
    /// The coordinates of the ring.
    pub ring: Vec<Coord<T>>,
}

impl<T> Default for Stream<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            rotate: RotateRadians::I(RotationIdentity::<T>::default()),
            ring: vec![],
        }
    }
}

impl<T> StreamTrait for Stream<T>
where
    T: CoordFloat + FloatConst,
{
    type EP = Self;
    type T = T;

    fn endpoint(&mut self) -> &mut Self {
        self
    }

    fn point(&mut self, p: &Coord<T>, _m: Option<u8>) {
        let x_rotated = &self.rotate.invert(p);
        self.ring.push(Coord {
            x: x_rotated.x.to_degrees(),
            y: x_rotated.y.to_degrees(),
        });
    }
}
