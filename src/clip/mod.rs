pub mod antimeridian;
pub mod buffer;
pub mod circle;
pub mod clip_base;

mod rejoin;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::path::PathResultEnum;
use crate::stream::StreamPathResult;

use buffer::ClipBuffer;

pub trait BufferInTrait // where
//     T: CoordFloat + FloatConst,
{
    // Box<
    //         dyn StreamPathResult<
    //             Out = Option<PathResultEnum<T>>,
    //             ScC = Coordinate<T>,
    //
    //         >,
    //     >,
    type BitSink;
    fn buffer_in(&mut self, _sink: Self::BitSink);
}
