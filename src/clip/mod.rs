pub mod antimeridian;
pub mod buffer;
pub mod circle;
pub mod clip_base;

mod rejoin;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::stream::StreamPathResultNode;

use buffer::ClipBuffer;

pub trait BufferInTrait<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn buffer_in(&mut self, _sink: StreamPathResultNode<T>) {
        panic!("Must override this");
    }
}
