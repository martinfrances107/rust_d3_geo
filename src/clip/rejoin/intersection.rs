use std::cell::RefCell;
use std::rc::Rc;

use derivative::Derivative;
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
    pub o: Option<Rc<RefCell<Intersection<T>>>>, // another intersection,
    pub e: bool, // is any entry?
    pub v: bool,
    #[derivative(Debug = "ignore")]
    pub n: Option<Rc<RefCell<Intersection<T>>>>, // next
    #[derivative(Debug = "ignore")]
    pub p: Option<Rc<RefCell<Intersection<T>>>>, // previous
}

impl<T> PartialEq for Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    /// Ignore potentially circular elements o, n, p
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.z == other.z && self.e == other.e && self.v == other.v
    }
}
impl<T> Eq for Intersection<T> where T: CoordFloat + FloatConst {}

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
