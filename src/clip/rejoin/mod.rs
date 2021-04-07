pub mod intersection;

use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::Zero;
use num_traits::{Float, FloatConst};

use crate::point_equal::point_equal;
use crate::stream::Stream;

use super::antimeridian::ClipAntimeridian;
use super::buffer::LineElem;
use super::clip_raw::ClipRaw;
use super::ClipTraitRaw;
use intersection::Intersection;

/// A generalized polygon clipping algorithm: given a polygon that has been cut
/// into its visible line segments, and rejoins the segments by interpolating
/// along the clip edge.
pub fn rejoin<T>(
    segments: &Vec<Vec<LineElem<T>>>,
    raw: ClipRaw<T>,
    start_inside: bool,
    stream: &mut impl Stream<T, C = Coordinate<T>>,
) where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    let mut start_inside = start_inside;
    let mut subject = Vec::<Intersection<T>>::new();
    let mut clip = Vec::<Intersection<T>>::new();

    for segment in segments.iter() {
        let n = segment.len() - 1usize;
        if n <= 0 {
            return;
        };
        let mut p0: LineElem<T> = segment[0];
        let mut p1: LineElem<T> = segment[n];

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

        let mut x = Intersection::new(p0, Some(segment.to_vec()), None, true);
        subject.push(x.clone());

        x.o = Some(Box::new(Intersection::new(
            p0,
            None,
            Some(Box::new(x.clone())),
            false,
        )));
        clip.push(*x.o.unwrap());

        x = Intersection::new(p1, Some(segment.to_vec()), None, false);
        subject.push(x.clone());

        x.o = Some(Box::new(Intersection::new(
            p1,
            None,
            Some(Box::new(x.clone())),
            true,
        )));
        clip.push(*x.clone().o.unwrap());
    }

    if subject.is_empty() {
        return;
    }

    clip.sort_by(ClipRaw::compare_intersection);

    link(&mut subject);
    link(&mut clip);

    for i in 0..clip.len() {
        start_inside = !start_inside;
        clip[i].e = start_inside;
    }

    let start = subject[0].clone();
    // let points: Vec<LineElem<T>>;
    let mut point;

    loop {
        // Find first unvisited intersection.
        let mut current = start.clone();
        let mut is_subject = true;

        while current.v {
            // current = current.n;
            // if current == start {
            //     return;
            // }
            match current.n {
                Some(n) => {
                    current = *n;
                    // must implement compare
                    // if current == start {
                    //     return;
                    // }
                }
                None => {}
            }
        }
        let mut points = current.z;

        stream.line_start();
        loop {
            match &mut current.o {
                Some(o) => {
                    o.v = true;
                }
                None => {
                    panic!("Cannot reach into blank and set true");
                }
            }
            current.v = true;

            if current.e {
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
                        Some(current.x.p),
                        Some(current.n.clone().unwrap().x.p),
                        T::one(),
                        stream,
                    );
                }
                current = *current.n.unwrap();
            } else {
                if is_subject {
                    points = current.p.clone().unwrap().z;
                    for i in (1..points.clone().unwrap().len()).rev() {
                        point = points.clone().unwrap()[i];
                        stream.point(&point.p, None);
                    }
                } else {
                    raw.interpolate(
                        Some(current.x.p),
                        Some(current.p.clone().unwrap().x.p),
                        T::from(-1).unwrap(),
                        stream,
                    );
                }
            }
            current = *current.p.unwrap();

            current = *current.o.unwrap();
            points = current.z;
            is_subject = !is_subject;

            if !current.v {
                break;
            }
        }
        stream.line_end();
    }
}

fn link<T>(array: &mut Vec<Intersection<T>>)
where
    T: CoordFloat + FloatConst,
{
    if array.is_empty() {
        return;
    };
    let n = array.len();

    let mut a = array[0].clone();
    let mut b: Intersection<T>;
    for i in 1..n {
        b = array[i].clone();
        a.n = Some(Box::new(b.clone()));
        b.p = Some(Box::new(a));
        a = b;
    }
    b = array[0].clone();
    a.n = Some(Box::new(b));

    todo!("With the  command below javascript is updating the input array -- so must fix here.");
    // b.p = Some(Box::new(a));
}
