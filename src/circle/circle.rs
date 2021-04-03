use std::fmt::Debug;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::StreamType;
use crate::rotation::rotate_radians_transform::RotateRadiansEnum;
use crate::rotation::rotation_identity::RotationIdentity;
use crate::stream::Stream;
use crate::stream::stream_dst::StreamDst;
use crate::Transform;
use crate::TransformIdentity;

/// Output of CircleGenertor::circle()
#[derive(Debug)]
pub struct CircleStream<T: CoordFloat + Default + FloatConst> {
    pub stream_type: StreamType,
    pub coordinates: Vec<Vec<Coordinate<T>>>,
    pub rotate: RotateRadiansEnum<T>,
    pub ring: Vec<Coordinate<T>>,
}

impl<T: CoordFloat + Default + FloatConst> Default for CircleStream<T> {
    fn default() -> Self {
        Self {
            stream_type: StreamType::Polygon,
            coordinates: vec![vec![]],
            // rotate: Box::new(TransformIdentity::default()),
            rotate: RotateRadiansEnum::I(RotationIdentity::default()),
            ring: vec![],
        }
    }
}

impl<T: CoordFloat + Default + FloatConst> Clone for CircleStream<T> {
    fn clone(&self) -> CircleStream<T> {
        CircleStream::<T> {
            stream_type: self.stream_type.clone(),
            coordinates: self.coordinates.clone(),
            rotate: self.rotate.clone(),
            ring: self.ring.clone(),
        }
    }
}

impl<T: AddAssign + CoordFloat + Default + FloatConst> Stream<T> for CircleStream<T> {
    type C = Coordinate<T>;
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        let p = p.clone();
        let rotate = &self.rotate;
        let x_rotated = rotate.invert(&p);
        let x_rotated_deg = Coordinate {
            x: x_rotated.x.to_degrees(),
            y: x_rotated.y.to_degrees(),
        };
        self.ring.push(x_rotated_deg);
    }
    fn sphere(&mut self) {}

    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}

    fn get_dst(&self) -> StreamDst<T> {
        StreamDst::Circle(self.clone())
    }
}
