use derivative::Derivative;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use std::fmt::Display;
use std::ops::AddAssign;
// use std::rc::Rc;

// use crate::clip::antimeridian::ClipAntimeridian;
// use crate::clip::interpolate_trait::Interpolate;
// use crate::clip::Clip;
use crate::rotation::rotate_radians_enum::RotateRadiansEnum;
// use crate::rotation::rotation_identity::RotationIdentity;
// use crate::stream::stream_in_trait::StreamCombo;
// use crate::stream::stream_in_trait::StreamIn;
// use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::Transform;

// use super::ProjectionRawTrait;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct StreamTransform<
    'a,
    // STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
>
//where
//     dyn Clip<T>: Clip<T, C = Coordinate<T>> + Interpolate<T, C = Coordinate<T>>,
{
    #[derivative(Debug = "ignore")]
    pub stream: Box<dyn 'a + Stream<SC = Coordinate<T>>>,
    pub transform: &'a RotateRadiansEnum<T>,
}

// impl<STREAM, T> Default for StreamTransform<STREAM, T>
// where
//     // P: Transform<TcC = Coordinate<T>>,
//     // PR: ProjectionRawTrait,
//     STREAM: Stream<SC = Coordinate<T>> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     fn default() -> Self {
//         Self {
//             transform: RotateRadiansEnum::I(RotationIdentity::default()),
//             stream: STREAM::default(),
//         }
//     }
// }

// impl<STREAM, T> StreamIn for StreamTransform<STREAM, T>
// where
//     STREAM: Stream<SC = Coordinate<T>> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     // type C = Coordinate<T>;
//     // type T = T;
//     // type SD = SD;
//     type SInput = STREAM;
//     #[inline]
//     fn stream_in(&mut self, stream: STREAM) {
//         self.stream = stream;
//     }
// }

impl<
        'a,
        // Rc<PR>: ProjectionRawTrait + Transform<C=Coordinate<T>>,
        // SD,
        // STREAM: Stream<SC = Coordinate<T>>,
        // STREAM: Stream<SC = Coordinate<T>>,
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    > StreamTransform<'a, T>
{
    #[inline]
    pub fn new(
        // projection_raw: &PR,
        transform: &'a RotateRadiansEnum<T>,
        stream: Box<dyn 'a + Stream<SC = Coordinate<T>>>,
    ) -> StreamTransform<'a, T>
where
        // Rc<PR>: Transform<C = Coordinate<T>>,
        // PR: Transform<C = Coordinate<T>>,
        // STREAM: Stream<SC = Coordinate<T>>,
    {
        {
            // let transform: RotateRadiansEnum<T>;

            // match transform_in {
            //     Some(t) => {
            //         transform = t;
            //     }
            //     None => {
            //         transform = RotateRadiansEnum::I(RotationIdentity::<T>::default());
            //     }
            // }

            Self {
                stream: stream,
                transform,
            }
        }
    }
}

impl<'a, T> Transform for StreamTransform<'a, T>
where
    // PR: Transform<C = Coordinate<T>>,
    // STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type C = Coordinate<T>;
    fn transform(&self, p: &Self::C) -> Self::C {
        self.transform.transform(p)
    }
    fn invert(&self, p: &Self::C) -> Self::C {
        self.transform.invert(p)
    }
}

impl<'a, T> Stream for StreamTransform<'a, T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
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
