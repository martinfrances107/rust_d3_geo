// use std::ops::AddAssign;

// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::FloatConst;

// use crate::projection::stream_transform_radians::StreamTransformRadians;
// use crate::stream::Stream;
// use crate::stream::StreamDst;

// #[derive(Debug)]
// pub struct StreamPipe<T>
// where
//     T: CoordFloat + FloatConst + Default,
// {
//     pub input: StreamTransformRadians<T>,
//     pub output: StreamDst<T>,
// }

// impl<T> StreamPipe<T>
// where
//     T: CoordFloat + FloatConst + Default,
// {
//     #[inline]
//     pub fn new(input: StreamTransformRadians<T>, output: StreamDst<T>) -> Self {
//         Self { input, output }
//     }
// }

// impl<T> Stream<T> for StreamPipe<T>
// where
//     T: AddAssign + CoordFloat + Default + FloatConst,
// {
//     type C = Coordinate<T>;

//     #[inline]
//     fn get_dst(&self) -> StreamDst<T> {
//         todo!("must sort this");
//     }

//     #[inline]
//     fn point(&mut self, p: &Self::C, m: Option<u8>) {
//         self.input.point(p, m);
//     }

//     #[inline]
//     fn sphere(&mut self) {
//         self.input.sphere();
//     }
//     #[inline]
//     fn line_start(&mut self) {
//         self.input.line_start();
//     }
//     #[inline]
//     fn line_end(&mut self) {
//         self.input.line_end();
//     }
//     #[inline]
//     fn polygon_start(&mut self) {
//         self.input.polygon_start();
//     }
//     #[inline]
//     fn polygon_end(&mut self) {
//         self.input.polygon_end();
//     }
// }
