use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::fmt::Debug;

use super::StreamType;
use crate::stream::Stream;
use crate::Transform;

/// Output of CircleGenertor::circle()
pub struct CircleStream<T: CoordFloat> {
    pub stream_type: StreamType,
    pub coordinates: Vec<Vec<Coordinate<T>>>,
    pub rotate: Box<dyn Transform<C = Coordinate<T>, TcC = Coordinate<T>>>,
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
