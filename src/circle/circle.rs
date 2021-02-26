use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::fmt::Debug;

use super::StreamType;
use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::Transform;
use crate::TransformIdentity;

/// Output of CircleGenertor::circle()
pub struct CircleStream<T: CoordFloat> {
    pub stream_type: StreamType,
    pub coordinates: Vec<Vec<Coordinate<T>>>,
    pub rotate: Box<dyn Transform<TcC = Coordinate<T>>>,
    pub ring: Vec<Coordinate<T>>,
}

impl<T: CoordFloat + FloatConst + Default + 'static> Default for CircleStream<T> {
    fn default() -> Self {
        Self {
            stream_type: StreamType::Polygon,
            coordinates: vec![vec![]],
            rotate: Box::new(TransformIdentity::default()),
            ring: vec![],
        }
    }
}

impl<T: CoordFloat + FloatConst + Default + 'static> Clone for CircleStream<T> {
    fn clone(&self) -> Self {
        Self {
            stream_type: self.stream_type,
            coordinates: self.coordinates.clone(),
            rotate: self.rotate.clone_box(),
            ring: self.ring.clone(),
        }
    }
}
impl<T: CoordFloat + FloatConst + Default + 'static> StreamClone for CircleStream<T> {
    type ScC = Coordinate<T>;
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(self.clone())
    }
}
impl<T: CoordFloat + FloatConst + Default + 'static> Stream for CircleStream<T> {
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        let x_rotated = self.rotate.invert(&p);
        let x_rotated_deg = Coordinate {
            x: x_rotated.x.to_degrees(),
            y: x_rotated.y.to_degrees(),
        };
        self.ring.push(x_rotated_deg);
    }
}
