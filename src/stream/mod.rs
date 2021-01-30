pub mod feature_collection;
pub mod geometry;
pub mod geometry_collection;
mod geometry_processor;
pub mod line;
pub mod line_string;
pub mod multi_line_string;
pub mod multi_point;
pub mod multi_polygon;
pub mod point;
pub mod polygon;

use geo::CoordFloat;
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

/// Applies to DataObject's
pub trait Streamable<T: CoordFloat + FloatConst> {
    fn to_stream(&self, stream: &mut impl Stream<T>);
}

/// Hold state associated with the input/output of a StreamProcessor.
pub type StreamNode<T> = Rc<RefCell<Box<dyn Stream<T>>>>;

pub trait Stream<T>
where
    T: CoordFloat + FloatConst,
{
    fn point(&mut self, _x: T, _y: T, _z: Option<u8>) {}
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
}
