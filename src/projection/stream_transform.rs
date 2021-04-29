use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::clip::Clip;
use crate::rotation::rotate_radians_transform::RotateRadiansEnum;
use crate::rotation::rotation_identity::RotationIdentity;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::Transform;

#[derive(Clone, Debug)]
pub struct StreamTransform<
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
> {
    pub transform: RotateRadiansEnum<T>,
    pub stream: Clip<T>,
}

impl<'a, T> Default for StreamTransform<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    fn default() -> Self {
        Self {
            transform: RotateRadiansEnum::I(RotationIdentity::default()),
            stream: ClipAntimeridian::gen_clip(),
        }
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst>
    StreamTransform<T>
{
    #[inline]
    pub fn stream_in(&mut self, stream: Clip<T>) {
        self.stream = stream;
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst>
    StreamTransform<T>
{
    #[inline]
    pub fn new(transform_in: Option<RotateRadiansEnum<T>>) -> StreamTransform<T> {
        {
            let transform: RotateRadiansEnum<T>;

            match transform_in {
                Some(t) => {
                    transform = t.clone();
                }
                None => {
                    transform = RotateRadiansEnum::I(RotationIdentity::<T>::default());
                }
            }

            Self {
                stream: ClipAntimeridian::gen_clip(),
                transform,
            }
        }
    }
}

impl<'a, T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Transform
    for StreamTransform<T>
{
    type TcC = Coordinate<T>;
    fn transform(&self, p: &Self::TcC) -> Self::TcC {
        self.transform.transform(p)
    }
    fn invert(&self, p: &Self::TcC) -> Self::TcC {
        self.transform.invert(p)
    }
}

impl<'a, T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Stream<T>
    for StreamTransform<T>
{
    type C = Coordinate<T>;

    #[inline]
    fn get_dst(&self) -> StreamDst<T> {
        self.stream.get_dst()
    }

    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        // Warning the javascript version return the value below but I think it break the implied spec!!!!
        self.stream.point(&self.transform(&p), m);
    }

    #[inline]
    fn sphere(&mut self) {
        self.stream.sphere();
    }

    #[inline]
    fn line_start(&mut self) {
        self.stream.line_start();
    }

    #[inline]
    fn line_end(&mut self) {
        self.stream.line_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.stream.polygon_start();
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.stream.polygon_end();
    }
}
