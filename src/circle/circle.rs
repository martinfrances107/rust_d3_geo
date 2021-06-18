use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::StreamType;
use crate::rotation::rotate_radians_enum::RotateRadiansEnum;
use crate::rotation::rotation_identity::RotationIdentity;
// use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::Transform;

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

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Stream
    for CircleStream<T>
{
    type SC = Coordinate<T>;
    // type ST = T;
    // type SD = CircleStream<T>;
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
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

    // #[inline]
    // fn get_dst(&self) -> Self {
    //     self.clone()
    // }
}
