use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::fmt::Debug;

use crate::rotation::rotate_radians::rotate_radians_transform;
use crate::stream::Stream;
use crate::stream::StreamDummy;
// use crate::stream::StreamSimpleNode;
use crate::Transform;
use crate::{cartesian::cartesian, TransformIdentity};
use crate::{cartesian::cartesian_normalize_in_place, stream::StreamIdentity};

use super::circle_stream::circle_stream;
use super::CircleInArg;
use super::CircleTrait;
use super::FnValMaybe;
use super::FnValMaybe2D;
use super::StreamType;
use std::cell::RefCell;
use std::rc::Rc;

/// Output of CircleGenertor::circle()
pub struct CircleStream<T: CoordFloat> {
    pub stream_type: StreamType,
    pub coordinates: Vec<Vec<Coordinate<T>>>,
    pub rotate: Box<dyn Transform<C = Coordinate<T>>>,
    pub ring: Vec<Coordinate<T>>,
}

impl<T: CoordFloat + FloatConst> Stream for CircleStream<T> {
    type C = Coordinate<T>;
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        let x_rotated = self.rotate.invert(&p);
        let x_rotated_deg = Coordinate {
            x: x_rotated.x.to_degrees(),
            y: x_rotated.y.to_degrees(),
        };
        self.ring.push(x_rotated_deg);
    }
}
