pub mod antimeridian;
pub mod buffer;
pub mod circle;
pub mod clip_base;

mod rejoin;

use crate::stream::Stream;
use crate::stream::StreamClipLineNode;
use crate::stream::StreamClipLineNodeStub;
use crate::stream::StreamPathResultNode;
use crate::stream::StreamPathResultNodeStub;

use buffer::{ClipBuffer, LineElem};
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

pub trait BufferInTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn buffer_in(&mut self, _sink: StreamPathResultNode<T>) {
        panic!("Must override this");
    }
}
