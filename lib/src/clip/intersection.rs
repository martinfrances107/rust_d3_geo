use core::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;

use crate::clip::line_elem::LineElem;

#[derive(Clone)]
pub(super) struct Intersection<'a, T>
where
    T: CoordFloat,
{
    pub x: LineElem<T>,
    pub z: Option<&'a Vec<LineElem<T>>>,
    /// Another intersection.
    pub o: Option<Rc<RefCell<Intersection<'a, T>>>>,

    /// Clip and subject polygons
    /// As we walk a "Clip"  polygon and intersect with "Subject" polygon are we exiting or entering?
    ///
    /// An equivalent question could be asked about walking "Subject" polygons.
    ///
    /// e - is it a  EXIT / (NOT EXIT)
    /// entry = true  means moving in a forward direction ( next )
    /// entry = false implies exit go back ( previous )
    pub e: bool,
    /// visited.
    pub v: bool,
    /// Next.
    pub n: Option<Rc<RefCell<Intersection<'a, T>>>>,
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
    ) -> Self {
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
