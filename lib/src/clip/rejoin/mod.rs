mod link;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use geo::CoordFloat;

use crate::abs_diff_eq;
use crate::clip::intersection::Intersection;
use crate::clip::rejoin::link::link;
use crate::clip::Interpolator;
use crate::math::EPSILON;
use crate::stream::Stream;

use super::line_elem::LineElem;

pub(super) type CompareIntersectionsFn<T> =
    Box<dyn Fn(&Rc<RefCell<Intersection<T>>>, &Rc<RefCell<Intersection<T>>>) -> Ordering>;

/// A generalized polygon clipping algorithm: given a polygon that has been cut
/// into its visible line segments, and rejoins the segments by interpolating
/// along the clip edge.
///
/// [see][https://www.inf.usi.ch/hormann/papers/Greiner.1998.ECO.pdf]
///
/// Variable definition.
///
/// A "Clip" polygon is cutting polygon.
/// A "Subject" polygon is the working peice to be cut.
///
/// A Clipped polygon is the result after the subject has be limited in extent
/// to no more than the boundary of the cutting polygon.
#[allow(clippy::too_many_lines)]
pub fn rejoin<CI, EP, INTERPOLATOR, SINK, T>(
    segments: &[Vec<LineElem<T>>],
    compare_intersection: CI,
    start_inside: bool,
    interpolator: &INTERPOLATOR,
    stream: &mut SINK,
) where
    CI: Fn(&Rc<RefCell<Intersection<T>>>, &Rc<RefCell<Intersection<T>>>) -> Ordering,
    SINK: Stream<EP = EP, T = T>,
    INTERPOLATOR: Interpolator<T = T>,
    T: CoordFloat,
{
    let mut start_inside = start_inside;
    let mut subject = Vec::<Rc<RefCell<Intersection<T>>>>::new();
    let mut clip = Vec::<Rc<RefCell<Intersection<T>>>>::new();

    let two_epsilon = T::from(2.0 * EPSILON).unwrap();
    for segment in segments.iter() {
        let (n, has_overflown) = segment.len().overflowing_sub(1_usize);
        if n == 0 || has_overflown {
            return;
        };

        let mut p0: LineElem<T> = segment[0];
        let mut p1: LineElem<T> = segment[n];

        if abs_diff_eq(&p0.p, &p1.p) {
            if p0.m.is_none() && p1.m.is_none() {
                stream.line_start();
                for elem in segment.iter().take(n) {
                    p0 = *elem;
                    stream.point(&p0.p, None);
                }
                stream.line_end();
                return;
            }
            // Handle degenerate cases by moving the point.
            p1.p.x = p1.p.x + two_epsilon;
        }

        let x1 = Rc::new(RefCell::new(Intersection::new(
            p0,
            Some(segment),
            None,
            true,
        )));
        subject.push(x1.clone());

        let o = Rc::new(RefCell::new(Intersection::new(
            p0,
            None,
            Some(x1.clone()),
            false,
        )));
        (*x1).borrow_mut().o = Some(o.clone());
        clip.push(o);

        let x2 = Rc::new(RefCell::new(Intersection::new(
            p1,
            Some(segment),
            None,
            false,
        )));
        subject.push(x2.clone());
        let o2 = Rc::new(RefCell::new(Intersection::new(
            p1,
            None,
            Some(x2.clone()),
            true,
        )));
        (*x2).borrow_mut().o = Some(o2.clone());
        clip.push(o2);
    }

    if subject.is_empty() {
        return;
    }

    clip.sort_by(compare_intersection);

    link(&mut subject);
    link(&mut clip);

    for c in clip {
        start_inside = !start_inside;
        c.borrow_mut().e = start_inside;
    }

    let start = &subject[0];

    loop {
        // current: First unprocessed intersection point
        // of the subject polygon.
        let mut current = start.clone();
        let mut is_subject = true;

        // Find first unvisited intersection.
        while current.borrow().v {
            current = current.clone().borrow().n.clone().unwrap();
            // The javascript as this point does === on two objects.
            // This is the same a comparison of raw points.
            if current.as_ptr() == (*start).as_ptr() {
                return;
            }
        }

        let mut points = current.borrow().z;

        stream.line_start();

        loop {
            current.borrow().o.clone().unwrap().borrow_mut().v = true;
            current.borrow_mut().v = true;
            if current.borrow().e {
                if is_subject {
                    points.map_or_else(
                        || {
                            todo!("how to do nothing here");
                        },
                        |points| {
                            for p in points {
                                stream.point(&p.p, None);
                            }
                        },
                    );
                } else {
                    interpolator.interpolate(
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
                        .z;
                    for le in points.unwrap().iter().rev() {
                        stream.point(&le.p, None);
                    }
                } else {
                    interpolator.interpolate(
                        Some((*current.clone()).borrow().x.p),
                        Some(
                            ((current.clone()).borrow().p.as_ref().unwrap())
                                .borrow()
                                .x
                                .p,
                        ),
                        -T::one(),
                        stream,
                    );
                }
                current = current.clone().borrow().p.clone().unwrap();
            }

            current = current.clone().borrow().o.clone().unwrap();
            points = current.borrow().z;

            is_subject = !is_subject;

            if current.borrow().v {
                break;
            }
        }
        stream.line_end();
    }
}
