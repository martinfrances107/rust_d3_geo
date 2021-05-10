use geo::CoordFloat;
use num_traits::{Float, FloatConst};
use std::cell::RefCell;
use std::rc::Rc;

use super::super::buffer::LineElem;

#[derive(Clone, Debug)]
pub struct Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    pub x: LineElem<T>,
    pub z: Option<Vec<LineElem<T>>>,
    pub o: Option<Rc<RefCell<Intersection<T>>>>, // another intersection,
    pub e: bool,                                 // is any entry?
    pub v: bool,                                 // visited
    pub n: Option<Rc<RefCell<Intersection<T>>>>, // next
    pub p: Option<Rc<RefCell<Intersection<T>>>>, // previous
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
