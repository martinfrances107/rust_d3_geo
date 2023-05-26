use std::cell::RefCell;
use std::rc::Rc;

use derivative::Derivative;
use geo::CoordFloat;

use crate::clip::line_elem::LineElem;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub(super) struct Intersection<'a, T>
where
    T: CoordFloat,
{
    pub x: LineElem<T>,
    pub z: Option<&'a Vec<LineElem<T>>>,
    #[derivative(Debug = "ignore")]
    /// Another intersection.
    pub o: Option<Rc<RefCell<Intersection<'a, T>>>>,
    /// e - is any entry?
    /// entry = true  means moving in a forward direction ( next )
    /// entry = false implies exit go back ( previous )
    pub e: bool,
    /// visited.
    pub v: bool,
    #[derivative(Debug = "ignore")]
    /// Next.
    pub n: Option<Rc<RefCell<Intersection<'a, T>>>>,
    #[derivative(Debug = "ignore")]
    /// Previous.
    pub p: Option<Rc<RefCell<Intersection<'a, T>>>>,
}

impl<'a, T> Intersection<'a, T>
where
    T: CoordFloat,
{
    pub const fn new(
        point: LineElem<T>,
        points: Option<&'a Vec<LineElem<T>>>,
        other: Option<Rc<RefCell<Intersection<'a, T>>>>,
        entry: bool,
    ) -> Intersection<'a, T> {
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
