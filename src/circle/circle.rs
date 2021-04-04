use std::fmt::Debug;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::StreamType;
use crate::rotation::rotate_radians_transform::RotateRadiansEnum;
use crate::rotation::rotation_identity::RotationIdentity;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
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
    #[inline]
    fn default() -> Self {
        Self {
            stream_type: StreamType::Polygon,
            coordinates: vec![vec![]],
            rotate: RotateRadiansEnum::I(RotationIdentity::default()),
            ring: vec![],
        }
    }
}

impl<T: CoordFloat + Default + FloatConst> Clone for CircleStream<T> {
    #[inline]
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
        let x_rotated = &self.rotate.invert(&p);
        self.ring.push(Coordinate {
            x: x_rotated.x.to_degrees(),
            y: x_rotated.y.to_degrees(),
        });
    }
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}

    #[inline]
    fn get_dst(&self) -> StreamDst<T> {
        StreamDst::Circle(self.clone())
    }
}
