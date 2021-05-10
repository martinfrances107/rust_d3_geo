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
use num_traits::Zero;

use crate::point_equal::point_equal;
use crate::stream::Stream;

use super::line_elem::LineElem;
use super::clip_raw::ClipRaw;
use super::ClipTraitRaw;
use intersection::Intersection;
use link::link;

/// A generalized polygon clipping algorithm: given a polygon that has been cut
/// into its visible line segments, and rejoins the segments by interpolating
/// along the clip edge.
pub fn rejoin<T>(
    segments: &Vec<Vec<LineElem<T>>>,
    raw: ClipRaw<T>,
    start_inside: bool,
    stream: &mut impl Stream<T, C = Coordinate<T>>,
) where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    println!("clip rejoin: segments: {:#?}", segments);
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
        println!("in segement loop p0 p1 {:?} {:?}", p0, p1);
        if point_equal(p0.p, p1.p) {
            if !p0.m.unwrap().is_zero() && !p1.m.unwrap().is_zero() {
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
            // p1[0] += 2F * f64::EPSILON;
            p1.p.x += T::from(2).unwrap() * T::epsilon();
        }

        let x1 = Rc::new(RefCell::new(Intersection::new(
            p0,
            Some(segment.to_vec()),
            None,
            true,
        )));
        subject.push(x1.clone());

        let other1 = Rc::new(RefCell::new(Intersection::new(
            p0,
            None,
            Some(x1.clone()),
            false,
        )));
        (*x1).borrow_mut().o = Some(other1.clone());

        x1.borrow_mut().o = Some(other1.clone());
        clip.push(other1);

        let x2 = Rc::new(RefCell::new(Intersection::new(
            p1,
            Some(segment.to_vec()),
            None,
            false,
        )));
        subject.push(x2.clone());
        let other2 = Rc::new(RefCell::new(Intersection::new(
            p1,
            None,
            Some(x1.clone()),
            false,
        )));
        (*x2).borrow_mut().o = Some(other2.clone());

        clip.push(other2);
    }

    if subject.is_empty() {
        return;
    }

    clip.sort_by(ClipRaw::compare_intersection);

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
        let mut current: Option<Rc<RefCell<Intersection<T>>>> = Some(start.clone());
        let mut is_subject = true;

        while current.clone().unwrap().borrow().v {
            current = match current.clone() {
                Some(c) => c.borrow().n.clone(),
                None => None,
            };

            match current.clone() {
                Some(c) => {
                    todo!("must implement compare.");
                    // if c == start {
                    //     return;
                    // }
                }
                None => { // No match}
                }
            }
        }
        let mut points = (current.clone().unwrap()).borrow().z.clone();

        stream.line_start();
        loop {
            match &mut (current.clone().unwrap()).borrow_mut().o {
                Some(o) => {
                    (*o).borrow_mut().v = true;
                }
                None => {
                    panic!("Cannot reach into blank and set true");
                }
            }
            (current.clone().unwrap()).borrow_mut().v = true;

            if (current.clone().unwrap()).borrow().e {
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
                        Some((current.clone().unwrap()).borrow().x.p),
                        Some(
                            (current.clone().unwrap())
                                .borrow()
                                .n
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .x
                                .p,
                        ),
                        T::one(),
                        stream,
                    );
                }
                let next = (*current.unwrap()).borrow().n.clone();
                current = next;
            } else {
                if is_subject {
                    points = (*(*current.clone().unwrap()).borrow().p.as_ref().unwrap())
                        .borrow()
                        .z
                        .clone();
                    for i in (1..points.clone().unwrap().len()).rev() {
                        point = points.clone().unwrap()[i];
                        stream.point(&point.p, None);
                    }
                } else {
                    raw.interpolate(
                        Some((*current.clone().unwrap()).borrow().x.p),
                        Some(
                            ((current.clone().unwrap()).borrow().p.as_ref().unwrap())
                                .borrow()
                                .x
                                .p,
                        ),
                        T::from(-1).unwrap(),
                        stream,
                    );
                }
            }

            current = Some(
                current
                    .clone()
                    .unwrap()
                    .borrow()
                    .p
                    .as_ref()
                    .unwrap()
                    .clone(),
            );
            current = match &current.clone().unwrap().borrow().p {
                Some(c) => Some(c.clone()),
                None => None,
            };

            points = match current.clone() {
                Some(c) => c.borrow().z.clone(),
                None => None,
            };
            is_subject = !is_subject;

            // if !(*current.clone().unwrap()).borrow().v {
            //     break;
            // }
            match current {
                Some(ref c) => {
                    if !(*c).borrow().v {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }
        stream.line_end();
    }
}
