pub mod intersection;

use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::Zero;
use num_traits::{Float, FloatConst};

use crate::point_equal::point_equal;
use crate::stream::Stream;

use super::buffer::LineElem;
use super::ClipTraitRaw;

use intersection::Intersection;

/// A generalized polygon clipping algorithm: given a polygon that has been cut
/// into its visible line segments, and rejoins the segments by interpolating
/// along the clip edge.
pub fn rejoin<T: Float>(
    segments: &Vec<Vec<LineElem<T>>>,
    raw: impl ClipTraitRaw<T>,
    start_inside: bool,    
    stream: &mut impl Stream<T, C = Coordinate<T>>,
) where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    let mut subject = Vec::<Intersection<T>>::new();
    let mut clip = Vec::<Intersection<T>>::new();

    for segment in segments.iter() {
        let n = segment.len() - 1usize;
        if n <= 0 {
            return;
        };
        let mut p0 = segment[0];
        let mut p1 = segment[n];

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

    // need access to raw.compare_intersection and raw.interpolate
    todo!("Major: complete this function.");
}

struct LinkNP<'a, T> {
    value: T,
    n: Option<&'a LinkNP<'a, T>>,
    p: Option<&'a LinkNP<'a, T>>,
}

fn link<T>(array: Vec<LineElem<T>>)
where
    T: CoordFloat + FloatConst,
{
    if array.is_empty() {
        return;
    };
    let n = array.len();

    let i: usize = 0usize;
    let mut a = LinkNP {
        value: array[0],
        n: None,
        p: None,
    };
    let mut b: LinkNP<LineElem<T>>;
    for i in 1..n {
        b = LinkNP {
            value: array[i],
            n: None,
            p: None,
        };
        // a.n = Some(&b);
        // b.p = Some(&a);
        a = b;
    }
    b = LinkNP {
        value: array[i],
        n: None,
        p: None,
    };
    // a.n = Some(&b);
    // b.p = Some(&a);
}
