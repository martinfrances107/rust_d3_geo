use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// use super::CompareIntersection;
use super::Stream;
use super::StreamClone;
// use super::StreamPostClipTrait;
// use super::StreamPreClipTrait;
// use super::StreamSimpleNode;
// use super::StreamDst;
// use crate::clip::ClipTraitRaw;
// use crate::projection::resample::resample::Resample;
// use crate::projection::resample::resample_none::ResampleNone;
// use crate::projection::resample::ResampleEnum;

#[derive(Clone, Copy, Debug, Default)]
pub struct StreamDummy<T>
where
    T: CoordFloat,
{
    /// Why the Phantom Data is required here...
    ///
    /// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
    /// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
    phantom: PhantomData<T>,
}
// impl<T> ClipTraitRaw for StreamDummy<T>
// where
//     T: CoordFloat + FloatConst + 'static,
// {
//     type SctC = Coordinate<T>;
//     type SctOC = Option<Coordinate<T>>;
//     type SctT = T;
//     // type SctStream = StreamSimpleNode<T>;
//     type SctCi = CompareIntersection<T>;

//     fn point_visible(&self, _p: Self::SctC, _z: Option<u8>) -> bool {
//         panic!("Calling point_visible on a stub");
//     }

//     fn interpolate(
//         &self,
//         _from: Self::SctOC,
//         _to: Self::SctOC,
//         _direction: Self::SctT,
//         // _stream: Self::SctStream,
//     ) {
//         panic!("Calling interpolate on a stub");
//     }
// }
// impl<T> StreamPreClipTrait for StreamDummy<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     type SpctResample = ResampleEnum<T>;
//     fn stream_resample_in(&mut self, _stream: Self::SpctResample) {
//         panic!("calling methods on a Dummy");
//     }
//     fn box_clone(
//         &self,
//     ) -> Box<
//         dyn StreamPreClipTrait<
//             SctC = Self::SctC,
//             SctCi = Self::SctCi,
//             SctOC = Self::SctOC,
//             SctT = Self::SctT,
//             SctStream = Self::SctStream,
//             SpctResample = Self::SpctResample,
//         >,
//     > {
//         panic!("call box_clone on a stub!");
//     }
// }

// impl<T> StreamPostClipTrait for StreamDummy<T>
// where
//     T: CoordFloat + FloatConst + 'static,
// {
//     type SpostctStream = StreamDst<T>;
//     fn stream_in(&mut self, _stream: Self::SpostctStream) {
//         panic!("call method on StreamDummy");
//     }
//     fn box_clone(
//         &self,
//     ) -> Box<
//         dyn StreamPostClipTrait<
//             SpostctStream = Self::SpostctStream,
//             C = Self::C,
//             SctC = Self::SctC,
//             SctOC = Self::SctOC,
//             SctT = Self::SctT,
//             SctCi = Self::SctCi,
//             SctStream = Self::SctStream,
//         >,
//     > {
//         panic!("calling box_clone on a dummy");
//     }
// }

// impl<T> StreamClone for StreamDummy<T>
// where
//     T: CoordFloat + FloatConst,
// {
//     type RetType = Box<dyn Stream<C = Coordinate<T>>>;
//     #[inline]
//     fn box_clone(&self) -> Self::RetType {
//         Box::new(self.clone())
//     }
// }

// impl<T> Stream<T> for StreamDummy<T>
// where
//     T: CoordFloat + Default + FloatConst,
// {
//     type C = Coordinate<T>;
// }

// impl<T> StreamInTrait<T> for StreamDummy<T> where T: CoordFloat + FloatConst {}
