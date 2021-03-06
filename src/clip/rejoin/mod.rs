pub mod intersection;
mod link;

use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::point_equal::point_equal;
use crate::stream::Stream;

use super::clip_raw::ClipRaw;
use super::line_elem::LineElem;
use super::ClipTraitRaw;
use intersection::Intersection;
use link::link;

/// A generalized polygon clipping algorithm: given a polygon that has been cut
/// into its visible line segments, and rejoins the segments by interpolating
/// along the clip edge.
pub fn rejoin<P: Clone, T>(
    segments: &Vec<Vec<LineElem<T>>>,
    raw: ClipRaw<P, T>,
    start_inside: bool,
    stream: &mut impl Stream<T, C = Coordinate<T>>,
) where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    let mut start_inside = start_inside;
    let mut subject = Vec::<Rc<RefCell<Intersection<T>>>>::new();
    let mut clip = Vec::<Rc<RefCell<Intersection<T>>>>::new();

    for segment in segments.iter() {
        let n = segment.len() - 1usize;
        if n <= 0 {
            return;
        };
        let mut p0: LineElem<T> = segment[0];
        let mut p1: LineElem<T> = segment[n];

        if point_equal(p0.p, p1.p) {
            if p0.m.is_none() && p1.m.is_none() {
                stream.line_start();
                // let i: usize;
                // for (i = 0; i < n; ++i) stream.point((p0 = segment[i])[0], p0[1]);
                for i in 0..n {
                    p0 = segment[i];
                    stream.point(&p0.p, None);
                }
                stream.line_end();
                return;
            }
            // handle degenerate cases by moving the point
            p1.p.x += T::from(2.0 * 1e-6).unwrap();
        }

        let x1 = Rc::new(RefCell::new(Intersection::new(
            p0,
            Some(segment.clone()),
            None,
            true,
        )));
        subject.push(x1.clone());

        (*x1).borrow_mut().o = Some(Rc::new(RefCell::new(Intersection::new(
            p0,
            None,
            Some(x1.clone()),
            false,
        ))));
        clip.push((*x1).borrow().o.clone().unwrap());

        let x2 = Rc::new(RefCell::new(Intersection::new(
            p1,
            Some(segment.clone()),
            None,
            false,
        )));
        subject.push(x2.clone());
        (*x2).borrow_mut().o = Some(Rc::new(RefCell::new(Intersection::new(
            p1,
            None,
            Some(x2.clone()),
            true,
        ))));
        clip.push((*x2).borrow().o.clone().unwrap());
    }

    if subject.is_empty() {
        return;
    }

    clip.sort_by(ClipRaw::<P, T>::compare_intersection);

    link(&mut subject);
    link(&mut clip);

    for i in 0..clip.len() {
        start_inside = !start_inside;
        (*clip[i]).borrow_mut().e = start_inside;
    }

    let start = &subject[0];
    // let points: Vec<LineElem<T>>;
    let mut point;

    loop {
        // Find first unvisited intersection.
        let mut current: Rc<RefCell<Intersection<T>>> = start.clone();
        let mut is_subject = true;

        while current.borrow().v {
            current = current.clone().borrow().n.clone().unwrap();
            if *current.borrow() == *start.borrow() {
                return;
            }
        }

        let mut points = current.borrow().z.clone();

        stream.line_start();
        loop {
            current.borrow().o.clone().unwrap().borrow_mut().v = true;
            current.borrow_mut().v = true;
            if current.borrow().e {
                if is_subject {
                    match points {
                        Some(points) => {
                            for i in 0..points.len() {
                                point = points[i];
                                stream.point(&point.p, point.m);
                            }
                        }
                        None => {}
                    }
                } else {
                    raw.interpolate(
                        Some((current.clone()).borrow().x.p),
                        Some((current.clone()).borrow().n.as_ref().unwrap().borrow().x.p),
                        T::one(),
                        stream,
                    );
                }
                current = current.clone().borrow().n.clone().unwrap();
            } else {
                if is_subject {
                    points = (*(*current.clone()).borrow().p.as_ref().unwrap())
                        .borrow()
                        .z
                        .clone();
                    for i in (1..points.clone().unwrap().len()).rev() {
                        point = points.clone().unwrap()[i];
                        stream.point(&point.p, None);
                    }
                } else {
                    raw.interpolate(
                        Some((*current.clone()).borrow().x.p),
                        Some(
                            ((current.clone()).borrow().p.as_ref().unwrap())
                                .borrow()
                                .x
                                .p,
                        ),
                        T::from(-1).unwrap(),
                        stream,
                    );
                }
                current = current.clone().borrow().p.clone().unwrap();
            }

            current = current.clone().borrow().o.clone().unwrap();
            points = current.clone().borrow().z.clone();

            is_subject = !is_subject;

            if current.borrow().v == true {
                break;
            }
        }
        stream.line_end();
    }
}
