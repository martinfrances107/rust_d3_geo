use geo::CoordFloat;
use num_traits::{Float, FloatConst};

use super::super::buffer::LineElem;

#[derive(Clone, Debug)]
pub struct Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    pub x: LineElem<T>,
    z: Option<Vec<LineElem<T>>>,
    pub o: Option<Box<Intersection<T>>>, // another intersection,
    e: bool,                             // is any entry?
    v: bool,                             // visited
    n: Option<LineElem<T>>,              // next
    p: Option<LineElem<T>>,              // previous
}

impl<T: Float> Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    pub fn new(
        point: LineElem<T>,
        points: Option<Vec<LineElem<T>>>,
        other: Option<Box<Intersection<T>>>,
        entry: bool,
    ) -> Self {
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
