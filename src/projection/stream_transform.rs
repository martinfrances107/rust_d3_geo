use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::clip::Clip;
use crate::rotation::rotate_radians_transform::RotateRadiansEnum;
use crate::rotation::rotation_identity::RotationIdentity;
use crate::stream::Stream;
use crate::stream::StreamDst;
use crate::Transform;

#[derive(Clone, Debug)]
pub struct StreamTransform<T: CoordFloat + Default + FloatConst> {
    pub transform: RotateRadiansEnum<T>,
    pub stream: Clip<T>,
}

impl<'a, T> Default for StreamTransform<T>
where
    T: CoordFloat + Default + FloatConst,
{
    fn default() -> Self {
        Self {
            transform: RotateRadiansEnum::I(RotationIdentity::default()),
            stream: ClipAntimeridian::gen_clip(),
        }
    }
}

impl<T: CoordFloat + Default + FloatConst> StreamTransform<T> {
    #[inline]
    pub fn stream_in(&mut self, stream: Clip<T>) {
        self.stream = stream;
    }
}

impl<T: CoordFloat + Default + FloatConst> StreamTransform<T> {
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

impl<'a, T: AddAssign + CoordFloat + Default + FloatConst> Transform for StreamTransform<T> {
    type TcC = Coordinate<T>;
    fn transform(&self, p: &Self::TcC) -> Self::TcC {
        self.transform.transform(p)
    }
    fn invert(&self, p: &Self::TcC) -> Self::TcC {
        self.transform.invert(p)
    }
}

impl<'a, T: AddAssign + CoordFloat + Default + FloatConst> Stream<T> for StreamTransform<T> {
    type C = Coordinate<T>;

    #[inline]
    fn get_dst(&self) -> StreamDst<T> {
        self.stream.get_dst()
    }

    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        // let r = self.transform(&p);
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
