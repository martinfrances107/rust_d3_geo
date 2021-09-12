pub mod intersection;
pub mod link;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::rejoin::link::link;
use crate::clip::InterpolateFn;
use crate::point_equal::point_equal;
use crate::stream::Stream;

use super::line_elem::LineElem;
use intersection::Intersection;

type CompareIntersectionsFn<T> =
    fn(a: &Rc<RefCell<Intersection<T>>>, b: &Rc<RefCell<Intersection<T>>>) -> Ordering;

// use link::link;
/// A generalized polygon clipping algorithm: given a polygon that has been cut
/// into its visible line segments, and rejoins the segments by interpolating
/// along the clip edge.
pub fn rejoin<SINK, T>(
    segments: &Vec<Vec<LineElem<T>>>,
    compare_intersections: CompareIntersectionsFn<T>,
    start_inside: bool,
    interpolate_fn: InterpolateFn<SINK, T>,
    stream: Rc<RefCell<SINK>>,
) where
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    let mut start_inside = start_inside;
    let mut subject = Vec::<Rc<RefCell<Intersection<T>>>>::new();
    let mut clip = Vec::<Rc<RefCell<Intersection<T>>>>::new();
    let mut stream_b = stream.borrow_mut();
    for segment in segments.iter() {
        let (n, has_overflown) = segment.len().overflowing_sub(1_usize);
        if n == 0 || has_overflown {
            return;
        };

        let mut p0: LineElem<T> = segment[0];
        let mut p1: LineElem<T> = segment[n];

        if point_equal(p0.p, p1.p) {
            if p0.m.is_none() && p1.m.is_none() {
                stream_b.line_start();
                // for i in 0..n {
                for elem in segment.iter().take(n) {
                    p0 = *elem;
                    stream_b.point(&p0.p, None);
                }
                stream_b.line_end();
                return;
            }
            // handle degenerate cases by moving the point
            p1.p.x = p1.p.x + T::from(2.0 * 1e-6).unwrap();
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

    clip.sort_by(compare_intersections);

    link(&mut subject);
    link(&mut clip);

    for c in clip {
        start_inside = !start_inside;
        c.borrow_mut().e = start_inside;
    }

    let start = &subject[0];
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

        stream.clone().borrow_mut().line_start();

        loop {
            current.borrow().o.clone().unwrap().borrow_mut().v = true;
            current.borrow_mut().v = true;
            if current.borrow().e {
                if is_subject {
                    match points {
                        Some(points) => {
                            for p in points {
                                point = p;
                                stream.clone().borrow_mut().point(&point.p, point.m);
                            }
                        }
                        None => {
                            todo!("how to do nothing here");
                        }
                    }
                } else {
                    interpolate_fn(
                        Some((current.clone()).borrow().x.p),
                        Some((current.clone()).borrow().n.as_ref().unwrap().borrow().x.p),
                        T::one(),
                        stream.clone(),
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
                        stream.borrow_mut().point(&point.p, None);
                    }
                } else {
                    interpolate_fn(
                        Some((*current.clone()).borrow().x.p),
                        Some(
                            ((current.clone()).borrow().p.as_ref().unwrap())
                                .borrow()
                                .x
                                .p,
                        ),
                        T::from(-1).unwrap(),
                        stream.clone(),
                    );
                }
                current = current.clone().borrow().p.clone().unwrap();
            }

            current = current.clone().borrow().o.clone().unwrap();
            points = current.clone().borrow().z.clone();

            is_subject = !is_subject;

            if current.borrow().v {
                break;
            }
        }
        stream.borrow_mut().line_end();
    }
}
