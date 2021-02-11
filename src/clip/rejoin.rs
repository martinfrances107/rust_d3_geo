use geo::CoordFloat;
use num_traits::{Float, FloatConst};

use crate::{point_equal::point_equal, stream::StreamSimpleNode};

use super::buffer::LineElem;
use num_traits::Zero;

struct Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    x: LineElem<T>,
    z: Option<Vec<LineElem<T>>>,
    o: Option<Box<Intersection<T>>>, // another intersection,
    e: bool,                         // is any entry?
    v: bool,                         // visited
    n: Option<LineElem<T>>,          // next
    p: Option<LineElem<T>>,          // previous
}

impl<T: Float> Intersection<T>
where
    T: CoordFloat + FloatConst,
{
    fn new(
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

/// A generalized polygon clipping algorithm: given a polygon that has been cut
/// into its visible line segments, and rejoins the segments by interpolating
/// along the clip edge.
pub fn rejoin<T: Float>(
    segments: &Vec<Vec<LineElem<T>>>,
    // compare_intersection: CompareIntersectionFn<T>,
    start_inside: bool,
    // interpolate: InterpolateFn<T>,
    mut stream: StreamSimpleNode<T>,
) where
    T: CoordFloat + FloatConst,
{
    let subject = Vec::<Intersection<T>>::new();
    let clip = Vec::<Intersection<T>>::new();
    // let i,
    // let n: usize;

    for segment in segments.iter() {
        let n = segment.len() - 1;
        if n <= 0 {
            return;
        };
        let mut p0 = segment[0];
        let mut p1 = segment[n];
        //  let mut x: Intersection<F>;

        if point_equal(p0.p, p1.p) {
            if !p0.m.unwrap().is_zero() && !p1.m.unwrap().is_zero() {
                let mut s = stream.borrow_mut();
                s.line_start();
                // let i: usize;
                // for (i = 0; i < n; ++i) stream.point((p0 = segment[i])[0], p0[1]);
                for i in 0..n {
                    p0 = segment[i];
                    s.point(p0.p, None);
                }
                s.line_end();
                return;
            }
            // handle degenerate cases by moving the point
            // p1[0] += 2F * f64::EPSILON;
            p1.p.x = p1.p.x + T::from(2).unwrap() * T::epsilon();
        }

        let mut x = Intersection::new(p0, Some(segment.to_vec()), None, true);
        subject.push(x);
        x.o = Some(Box::new(Intersection::new(
            p0,
            None,
            Some(Box::new(x)),
            false,
        )));
        clip.push(*x.o.unwrap());
        x = Intersection::new(p1, Some(segment.to_vec()), None, false);
        subject.push(x);
        // x.o = Some(Box::new(Intersection::new(
        //   p1,
        //   None,
        // //   Some(Box::rejoin::new(x)),
        //   true,
        // )));
        clip.push(*x.o.unwrap());
    }
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
