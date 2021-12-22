use std::cell::RefCell;
use std::rc::Rc;

use derivative::*;
use geo::CoordFloat;
use num_traits::{Float, FloatConst};

use crate::clip::line_elem::LineElem;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    pub x: LineElem<T>,
    pub z: Option<Vec<LineElem<T>>>,
    #[derivative(Debug = "ignore")]
    /// Another intersection.
    pub o: Option<Rc<RefCell<Intersection<T>>>>,
    /// is any entry?
    pub e: bool,
    /// visited.
    pub v: bool,
    #[derivative(Debug = "ignore")]
    /// Next.
    pub n: Option<Rc<RefCell<Intersection<T>>>>,
    #[derivative(Debug = "ignore")]
    /// Previous.
    pub p: Option<Rc<RefCell<Intersection<T>>>>,
}

impl<T: Float> Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    pub fn new(
        point: LineElem<T>,
        points: Option<Vec<LineElem<T>>>,
        other: Option<Rc<RefCell<Intersection<T>>>>,
        entry: bool,
    ) -> Intersection<T> {
        Self {
            x: point,
            z: points,
            o: other,
            e: entry,
            v: false,
            n: None,
            p: None,
        }
    }
}
