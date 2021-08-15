use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::{Float, FloatConst};

use crate::clip::line_elem::LineElem;
use derivative::Derivative;

#[derive(Clone)]
pub struct Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    pub x: LineElem<T>,
    pub z: Option<Vec<LineElem<T>>>,
    // #[derivative(Debug = "ignore")]
    pub o: Option<Rc<RefCell<Intersection<T>>>>, // another intersection,
    pub e: bool,                                 // is any entry?
    pub v: bool,
    // #[derivative(Debug = "ignore")]
    pub n: Option<Rc<RefCell<Intersection<T>>>>, // next
    // #[derivative(Debug = "ignore")]
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

/// Cannot auto derive debug.
///
/// Once link() is called on a array of Intersection(s)
/// n, p by design become circular!
/// In that state the auto-derive version overflows the stack.
impl<T> fmt::Debug for Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Intersection")
            .field("x", &self.x)
            .field("z", &self.z)
            //  .field("o", "maybe circular")
            .field("e", &self.e)
            .field("v", &self.v)
            // ommit fields n , p
            .finish()
    }
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
        return Self {
            x: point,
            z: points,
            o: other,
            e: entry,
            v: false,
            n: None,
            p: None,
        };
    }
}
