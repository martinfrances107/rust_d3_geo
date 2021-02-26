use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::path::PathResultEnum;
use crate::point_equal::point_equal;
use crate::stream::StreamClean;
// use crate::stream::StreamClipLineNode;
use crate::stream::StreamClone;
use crate::stream::StreamDummy;
use crate::stream::StreamPathResult;
use crate::stream::StreamSimpleNode;
use crate::stream::{Clean, CleanEnum, Stream};

use super::intersect::intersect;
use super::intersect::IntersectReturn;
use super::BufferInTrait;

pub struct Line<T: CoordFloat> {
    c0: u8,           // code for previous point
    clean: CleanEnum, // no intersections
    radius: T,
    cr: T,
    not_hemisphere: bool,
    // point0: (Option<Point>, Option<u8>), // previous point with message.
    point0: Option<Coordinate<T>>, // previous point
    small_radius: bool,
    stream: Box<dyn Stream<ScC = Coordinate<T>>>,
    v0: bool,  // visibility of previous point
    v00: bool, // visibility of first point
}

impl<T> Default for Line<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn default() -> Self {
        Self {
            c0: 0u8,
            clean: CleanEnum::IntersectionsOrEmpty,
            radius: T::zero(),
            cr: T::zero(),
            not_hemisphere: false,
            point0: None,
            small_radius: false,
            stream: Box::new(StreamDummy::default()),
            v0: false,
            v00: false,
        }
    }
}

impl<T> BufferInTrait for Line<T>
where
    T: CoordFloat + FloatConst,
{
    type BitSink = Box<dyn StreamPathResult<Out = Option<PathResultEnum<T>>, ScC = Coordinate<T>>>;
    #[inline]
    fn buffer_in(&mut self, _sink: Self::BitSink) {
        // No-op.
    }
}
use crate::stream::StreamClipLine;
impl<T> StreamClipLine for Line<T> where T: CoordFloat + FloatConst + Default + 'static {}

impl<T: CoordFloat + FloatConst + Default + 'static> Line<T> {
    #[inline]
    pub fn new(radius: T) -> Self {
        // TODO small_radius, rc  is a shadow variables!!!
        let rc = radius.cos();
        let small_radius = rc.is_sign_positive();
        Self {
            c0: 0,
            clean: CleanEnum::IntersectionsOrEmpty,
            not_hemisphere: rc.abs() > T::epsilon(),
            point0: None,
            cr: T::zero(),
            radius,
            small_radius,
            v0: false,
            v00: false,
            // TOD pre or post clip.
            stream: Box::new(StreamDummy::default()),
        }
    }

    // #[inline]
    // pub fn gen_node(
    //     radius: T,
    // ) -> Box<dyn StreamClipLine<ScC = Coordinate<T>, BitSink = Box<dyn Stream<ScC = Coordinate<T>>>>>
    // {
    //     Box::new(Self::new(radius))
    // }

    #[inline]
    fn point_visible(&self, p: Coordinate<T>, _m: Option<u8>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }

    /// Generates a 4-bit vector representing the location of a point relative to
    /// the small circle's bounding box.
    const CODE_LEFT: u8 = 1;
    const CODE_RIGHT: u8 = 2;
    const CODE_BELOW: u8 = 4;
    const CODE_ABOVE: u8 = 8;
    fn code(&self, p: Coordinate<T>) -> u8 {
        let lambda = p.x;
        let phi = p.y;
        let r = match self.small_radius {
            true => self.radius,
            false => T::PI() - self.radius,
        };
        let mut code = 0;
        if lambda < -r {
            code |= Self::CODE_LEFT;
        } else if lambda > r {
            code |= Self::CODE_RIGHT;
        }
        if phi < -r {
            code |= Self::CODE_BELOW;
        } else if phi > r {
            code |= Self::CODE_ABOVE;
        }
        return code;
    }
}
impl<T> StreamClean<T> for Line<T> where T: CoordFloat + FloatConst + Default + 'static {}
impl<T> Clean for Line<T>
where
    T: CoordFloat + FloatConst,
{
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    #[inline]
    fn clean(&self) -> CleanEnum {
        match self.clean {
            CleanEnum::IntersectionsOrEmpty => CleanEnum::IntersectionsOrEmpty,
            CleanEnum::NoIntersections | CleanEnum::IntersectionsRejoin => {
                if self.v00 && self.v0 {
                    CleanEnum::IntersectionsRejoin
                } else {
                    CleanEnum::IntersectionsOrEmpty
                }
            }
        }
    }
}
impl<T: CoordFloat + FloatConst + Default + 'static> StreamClone for Line<T> {
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(*self.clone())
    }
}

impl<T: CoordFloat + FloatConst + Default + 'static> Stream for Line<T> {
    fn line_start(&mut self) {
        self.v00 = false;
        self.v0 = false;
        self.clean = CleanEnum::NoIntersections;
    }

    fn point(&mut self, p: Coordinate<T>, _m: Option<u8>) {
        let mut point1 = p;

        // let point2: (Option::<Point>, <Option<u8>>);
        let mut point2;
        let v = self.point_visible(p, None);

        let c = match self.small_radius {
            true => match v {
                true => 0u8,
                false => self.code(p),
            },
            false => match v {
                true => {
                    let inc = match p.x < T::zero() {
                        true => T::PI(),
                        false => -T::PI(),
                    };
                    self.code(Coordinate {
                        x: p.x + inc,
                        y: p.y,
                    })
                }
                false => 0u8,
            },
        };

        if self.point0.is_none() {
            self.v00 = v;
            self.v0 = v;
            if v {
                // let mut s = self.stream.borrow_mut();
                self.stream.line_start();
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
                    panic!("Requested One or None found Two as !!");
                }
            }
        }

        // let mut s = self.stream.borrow_mut();
        if v != self.v0 {
            let next: Option<Coordinate<T>>;
            self.clean = CleanEnum::IntersectionsOrEmpty;
            if v {
                // outside going in
                self.stream.line_start();
                match intersect(point1, self.point0.clone().unwrap(), self.cr, false) {
                    IntersectReturn::None => {
                        // TODO Should I do a stream Point here??
                        next = None;
                    }
                    IntersectReturn::One(p) => {
                        self.stream.point(p, None);
                        next = Some(p);
                    }
                    IntersectReturn::Two([p, _]) => {
                        self.stream.point(p, None);
                        // p0_next = p;
                        panic!("Silently dropping second point.");
                    }
                }
            } else {
                // inside going out
                point2 = intersect(self.point0.clone().unwrap(), point1, self.cr, false);
                match point2 {
                    IntersectReturn::None => {
                        // TODO should I stream a null point here?
                        // stream.line_end(); ???
                        panic!("Must deal with no intersect.");
                    }
                    IntersectReturn::One(p) => {
                        self.stream.point(p, Some(2));
                        self.stream.line_end();
                        next = Some(p);
                    }
                    IntersectReturn::Two([p, _]) => {
                        self.stream.point(p, Some(2));
                        self.stream.line_end();
                        // next = p;
                        panic!("Silently dropping second point.");
                    }
                }
            }
            self.point0 = next;
        } else if self.not_hemisphere && self.point0.is_none() && self.small_radius ^ v {
            // If the codes for two points are different, or are both zero,
            // and there this segment intersects with the small circle.
            if (c & self.c0) != 0 {
                let t = intersect(point1, self.point0.unwrap(), self.cr, true);
                match t {
                    IntersectReturn::None => {}
                    IntersectReturn::One(_) => {
                        panic!("Requeted two received one.");
                    }
                    IntersectReturn::Two(t) => {
                        self.clean = CleanEnum::IntersectionsOrEmpty;
                        if self.small_radius {
                            self.stream.line_start();
                            self.stream.point(t[0], None);
                            self.stream.point(t[1], None);
                            self.stream.line_end();
                        } else {
                            self.stream.point(t[1], None);
                            self.stream.line_end();
                            self.stream.line_start();
                            self.stream.point(t[0], Some(3u8));
                        }
                    }
                }
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
            // let mut s = self.strewam.borrow_mut();
            self.stream.line_end();
        }
        self.point0 = None;
    }
}
