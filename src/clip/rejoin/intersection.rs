use geo::CoordFloat;
use num_traits::{Float, FloatConst};

use super::super::buffer::LineElem;

#[derive(Clone, Debug)]
pub struct Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    pub x: LineElem<T>,
    pub z: Option<Vec<LineElem<T>>>,
    pub o: Option<Box<Intersection<T>>>, // another intersection,
    pub e: bool,                         // is any entry?
    pub v: bool,                         // visited
    pub n: Option<Box<Intersection<T>>>, // next
    pub p: Option<Box<Intersection<T>>>, // previous
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
