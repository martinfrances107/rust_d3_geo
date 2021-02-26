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

pub trait BufferInTrait {
    // Box<
    //         dyn StreamPathResult<
    //             Out = Option<PathResultEnum<T>>,
    //             ScC = Coordinate<T>,
    //
    //         >,
    //     >,
    type BitCB;
    fn buffer_in(&mut self, buffer: Self::BitCB);
}
