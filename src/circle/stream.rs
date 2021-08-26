use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::rotation::rotate_radians::RotateRadiams;
use crate::rotation::rotation_identity::RotationIdentity;
use crate::stream::Stream as StreamTrait;
use crate::Transform;

/// Output of CircleGenertor::circle()
#[derive(Clone, Debug)]
pub struct Stream<T>
where
    T: CoordFloat + FloatConst,
{
    // pub stream_type: StreamType,
    pub coordinates: Vec<Vec<Coordinate<T>>>,
    pub rotate: RotateRadiams<T>,
    pub ring: Vec<Coordinate<T>>,
}

impl<T> Default for Stream<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn default() -> Self {
        Self {
            // stream_type: StreamType::Polygon,
            coordinates: vec![vec![]],
            rotate: RotateRadiams::I(RotationIdentity::<T>::default()),
            ring: vec![],
        }
    }
}

impl<T> StreamTrait for Stream<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        let x_rotated = &self.rotate.invert(p);
        self.ring.push(Coordinate {
            x: x_rotated.x.to_degrees(),
            y: x_rotated.y.to_degrees(),
        });
    }
}
