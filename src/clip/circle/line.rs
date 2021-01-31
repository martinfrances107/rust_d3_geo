use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::point_equal::point_equal;
use crate::stream::Stream;
use crate::stream::StreamNode;
use crate::{clip::PointVisibleFn, transform_stream::StreamProcessor};
// use crate::transform_stream::StreamProcessor;

use super::intersect::intersect;
use super::intersect::IntersectReturn;

use std::cell::RefCell;
use std::rc::Rc;

// Takes a line and cuts into visible segments. Return values used for polygon
// clipPIng: 0 - there were intersections or the line was empty; 1 - no
// intersections 2 - there were intersections, and the first and last segments
// should be rejoined.

pub struct Line<T: CoordFloat> {
    c0: u8,    // code for previous point
    clean: u8, // no intersections
    radius: T,
    rc: T,
    not_hemisphere: bool,
    // point0: (Option<Point>, Option<u8>), // previous point with message.
    point0: Option<Coordinate<T>>, // previous point
    small_radius: bool,
    stream: StreamNode<T>,
    v0: bool,  // visibility of previous point
    v00: bool, // visibility of first point
    visible: PointVisibleFn<T>,
}

impl<T: CoordFloat + FloatConst + 'static> Line<T> {
    pub fn new(visible: PointVisibleFn<T>, radius: T) -> StreamProcessor<T> {
        Box::new(move |stream: StreamNode<T>| {
            // TODO small_radius, rc  is a shadow variables!!!
            let rc = radius.cos();
            let small_radius = rc.is_sign_positive();
            Rc::new(RefCell::new(Box::new(Line {
                c0: 0,
                clean: 0,
                not_hemisphere: rc.abs() > T::epsilon(),
                point0: None,
                rc,
                radius,
                small_radius,
                v0: false,
                v00: false,
                stream,
                visible: visible.clone(),
            })))
        })
    }

    /// Generates a 4-bit vector representing the location of a point relative to
    /// the small circle's bounding box.
    fn code(&self, lambda: T, phi: T) -> u8 {
        let r = match self.small_radius {
            true => self.radius,
            false => T::PI() - self.radius,
        };
        let mut code = 0;
        if lambda < -r {
            code |= 1;
        }
        // left
        else if lambda > r {
            code |= 2;
        } // right
        if phi < -r {
            code |= 4;
        }
        // below
        else if phi > r {
            code |= 8;
        } // above
        return code;
    }

    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    #[inline]
    fn clean(&self) -> u8 {
        self.clean | (((self.v00 && self.v0) as u8) << 1)
    }
}

impl<T: CoordFloat + FloatConst + 'static> Stream<T> for Line<T> {
    fn line_start(&mut self) {
        self.v00 = false;
        self.v0 = false;
        self.clean = 1;
    }

    fn point(&mut self, lambda: T, phi: T, _m: Option<u8>) {
        let mut point1 = Coordinate { x: lambda, y: phi };

        // let point2: (Option::<Point>, <Option<u8>>);
        let mut point2;
        let v = (self.visible)(lambda, phi, None);

        let c = match self.small_radius {
            true => match v {
                true => 0u8,
                false => self.code(lambda, phi),
            },
            false => match v {
                true => {
                    let inc = match lambda < T::zero() {
                        true => T::PI(),
                        false => -T::PI(),
                    };
                    self.code(lambda + inc, phi)
                }
                false => 0u8,
            },
        };

        if self.point0.is_none() {
            self.v00 = v;
            self.v0 = v;
            if v {
                let mut s = self.stream.borrow_mut();
                s.line_start();
            }
        }

        if v != self.v0 {
            point2 = intersect(
                self.point0.clone().unwrap(),
                point1.clone(),
                self.radius.cos(),
                false,
            );
            match point2 {
                IntersectReturn::None => {
                    point1.x = T::one();
                }
                IntersectReturn::One(p) => {
                    if point_equal(self.point0.clone().unwrap(), p.clone())
                        || point_equal(point1.clone(), p)
                    {
                        point1.x = T::one();
                    }
                }
                IntersectReturn::Two(_t) => {
                    // There is a subtle bug in the javascript here two points is handles
                    // as if the second does not exits.
                    // For now just cause a panic here to see how many times it occurs.
                    panic!("requested One or None found Two as !!");
                }
            }
        }

        let mut s = self.stream.borrow_mut();
        if v != self.v0 {
            let next: Option<Coordinate<T>>;
            self.clean = 0;
            if v {
                // outside going in
                s.line_start();
                match intersect(point1, self.point0.clone().unwrap(), self.rc, false) {
                    IntersectReturn::None => {
                        // TODO Should I do a stream Point here??
                        next = None;
                    }
                    IntersectReturn::One(p) => {
                        s.point(p.x, p.y, None);
                        next = Some(p);
                    }
                    IntersectReturn::Two([p, _]) => {
                        s.point(p.x, p.y, None);
                        // p0_next = p;
                        panic!("silently dropping second point");
                    }
                }
            } else {
                // inside going out
                point2 = intersect(self.point0.clone().unwrap(), point1, self.rc, false);
                match point2 {
                    IntersectReturn::None => {
                        // TODO should I stream a null point here?
                        // stream.line_end(); ???
                        panic!("Must deal with no intersect.");
                    }
                    IntersectReturn::One(p) => {
                        s.point(p.x, p.y, Some(2));
                        s.line_end();
                        next = Some(p);
                    }
                    IntersectReturn::Two([p, _]) => {
                        s.point(p.x, p.y, Some(2));
                        s.line_end();
                        // next = p;
                        panic!("silently dropping second point");
                    }
                }
            }
            self.point0 = next;
        } else if self.not_hemisphere && self.point0.is_none() && self.small_radius ^ v {
            // If the codes for two points are different, or are both zero,
            // and there this segment intersects with the small circle.
            if (c & self.c0) != 0 {
                // let t = intersect(point1.0.unwrap(), self.point0.0.unwrap(), self.rc, true);
                // match t {
                //   Return::None => {}
                //   Return::One(_) => {
                //     panic!("requetsed two received one");
                //   }
                //   Return::Two(t) => {
                //     self.clean = 0;
                //     if self.small_radius {
                //       stream.line_start();
                //       stream.point(t[0].x, t[0].y, None);
                //       stream.point(t[1].x, t[1].y, None);
                //       stream.line_end();
                //     } else {
                //       stream.point(t[1].x, t[1].y, None);
                //       stream.line_end();
                //       stream.line_start();
                //       stream.point(t[0].x, t[0].y, Some(3u8));
                //     }
                //   }
                // }
            }
        }
        // if v && self.point0.0.is_none() || !point_equal(self.point0.0.unwrap(), point1.0.unwrap()) {
        //   stream.point(point1.0.unwrap().x, point1.0.unwrap().y, None);
        // }
        // self.point0 = point1;
        self.v0 = v;
        self.c0 = c;
    }

    fn line_end(&mut self) {
        if self.v0 {
            let mut s = self.stream.borrow_mut();
            s.line_end();
        }
        self.point0 = None;
    }
}
