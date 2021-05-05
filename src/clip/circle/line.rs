use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::intersect::intersect;
use super::intersect::IntersectReturn;

use crate::clip::buffer::ClipBuffer;
use crate::clip::clip_sink_enum::ClipSinkEnum;
use crate::clip::line_sink_enum::LineSinkEnum;
use crate::point_equal::point_equal;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::stream::StreamSourceDummy;
use crate::stream::{Clean, CleanEnum};

#[derive(Clone, Debug)]
pub struct Line<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> {
    c0: u8,           // code for previous point
    clean: CleanEnum, // no intersections
    radius: T,
    cr: T,
    not_hemisphere: bool,
    point0: Option<Coordinate<T>>, // previous point
    small_radius: bool,
    stream: LineSinkEnum<T>,
    v0: bool,  // visibility of previous point
    v00: bool, // visibility of first point
}

impl<T> Default for Line<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    fn default() -> Self {
        Self {
            c0: 0u8,
            clean: CleanEnum::Undefined,
            radius: T::zero(),
            cr: T::zero(),
            not_hemisphere: false,
            point0: None,
            small_radius: false,
            stream: LineSinkEnum::CSE(ClipSinkEnum::Src(StreamDst::SRC(
                StreamSourceDummy::default(),
            ))),
            v0: false,
            v00: false,
        }
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Line<T> {
    #[inline]
    pub fn new(radius: T) -> Self {
        // TODO small_radius, rc  is a shadow variables!!!
        let cr = radius.cos();
        let small_radius = cr.is_sign_positive();
        Self {
            c0: 0,
            clean: CleanEnum::IntersectionsOrEmpty,
            not_hemisphere: cr.abs() > T::epsilon(),
            point0: None,
            cr,
            radius,
            small_radius,
            v0: false,
            v00: false,
            stream: LineSinkEnum::CB(ClipBuffer::default()),
        }
    }

    #[inline]
    pub fn stream_in(&mut self, stream: LineSinkEnum<T>) {
        self.stream = stream;
    }

    #[inline]
    pub fn get_stream(&self) -> LineSinkEnum<T> {
        self.stream.clone()
    }

    #[inline]
    fn visible(&self, p: &Coordinate<T>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }

    /// Generates a 4-bit vector representing the location of a point relative to
    /// the small circle's bounding box.
    const CODE_LEFT: u8 = 1;
    const CODE_RIGHT: u8 = 2;
    const CODE_BELOW: u8 = 4;
    const CODE_ABOVE: u8 = 8;
    fn code(&self, p: &Coordinate<T>) -> u8 {
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

impl<T> Clean for Line<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    #[inline]
    fn clean(&self) -> CleanEnum {
        println!("line(c) clean() ");
        match self.clean {
            CleanEnum::IntersectionsOrEmpty => CleanEnum::IntersectionsOrEmpty,
            CleanEnum::NoIntersections | CleanEnum::IntersectionsRejoin => {
                if self.v00 && self.v0 {
                    CleanEnum::IntersectionsRejoin
                } else {
                    CleanEnum::IntersectionsOrEmpty
                }
            }
            CleanEnum::Undefined => panic!("Should not clean an undefined value."),
        }
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Stream<T>
    for Line<T>
{
    type C = Coordinate<T>;
    fn sphere(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}

    fn get_dst(&self) -> StreamDst<T> {
        match &self.stream {
            LineSinkEnum::CB(cb) => cb.get_dst(),
            LineSinkEnum::CSE(cse) => match cse {
                ClipSinkEnum::Blank => {
                    panic!("calling get_dst on a blank");
                }
                ClipSinkEnum::Resample(r) => r.get_dst(),
                ClipSinkEnum::Src(s) => s.get_dst(),
            },
        }
    }
    fn line_start(&mut self) {
        self.v00 = false;
        self.v0 = false;
        self.clean = CleanEnum::NoIntersections;
    }

    fn point(&mut self, p: &Self::C, _m: Option<u8>) {
        let mut point1 = p.clone();
        let mut point2;
        let v = self.visible(p);

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
                    self.code(&Coordinate {
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
                match &mut self.stream {
                    LineSinkEnum::CB(stream) => {
                        stream.line_start();
                    }
                    LineSinkEnum::CSE(stream) => match stream {
                        ClipSinkEnum::Blank => {
                            panic!("ClickSinkEnum - actively using an unconnected blank");
                        }
                        ClipSinkEnum::Resample(stream) => stream.line_start(),
                        ClipSinkEnum::Src(stream) => stream.line_start(),
                    },
                }
            }
        }
        if v != self.v0 {
            point2 = intersect(
                &self.point0.clone().unwrap(),
                &point1,
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

            if v != self.v0 {
                let next: Option<Coordinate<T>>;
                self.clean = CleanEnum::IntersectionsOrEmpty;
                if v {
                    // outside going in
                    match self.stream.clone() {
                        LineSinkEnum::CB(mut stream) => {
                            stream.line_start();
                        }
                        LineSinkEnum::CSE(stream) => match stream {
                            ClipSinkEnum::Blank => {
                                panic!("ClickSinkEnum - actively using an unconnected blank");
                            }
                            ClipSinkEnum::Resample(mut stream) => {
                                stream.line_start();
                            }
                            ClipSinkEnum::Src(mut stream) => {
                                stream.line_start();
                            }
                        },
                    }
                    match intersect(&point1, &self.point0.clone().unwrap(), self.cr, false) {
                        IntersectReturn::None => {
                            // TODO Should I do a stream Point here??
                            next = None;
                        }
                        IntersectReturn::One(p) => {
                            // self.stream.point(p, None);
                            match self.stream.clone() {
                                LineSinkEnum::CB(mut stream) => {
                                    stream.point(&p, None);
                                }
                                LineSinkEnum::CSE(stream) => {
                                    match stream {
                                        ClipSinkEnum::Blank => {
                                            panic!("ClickSinkEnum - actively using an unconnected blank");
                                        }
                                        ClipSinkEnum::Resample(mut stream) => {
                                            stream.point(&p, None);
                                        }
                                        ClipSinkEnum::Src(mut stream) => {
                                            stream.point(&p, None);
                                        }
                                    }
                                }
                            }
                            next = Some(p);
                        }
                        IntersectReturn::Two([p, _]) => {
                            match self.stream.clone() {
                                LineSinkEnum::CB(mut stream) => {
                                    stream.point(&p, None);
                                }
                                LineSinkEnum::CSE(stream) => {
                                    match stream {
                                        ClipSinkEnum::Blank => {
                                            panic!("ClickSinkEnum - actively using an unconnected blank");
                                        }
                                        ClipSinkEnum::Resample(mut stream) => {
                                            stream.point(&p, None);
                                        }
                                        ClipSinkEnum::Src(mut stream) => {
                                            stream.point(&p, None);
                                        }
                                    }
                                }
                            }
                            // p0_next = p;
                            panic!("Silently dropping second point.");
                        }
                    }
                } else {
                    // inside going out
                    point2 = intersect(&self.point0.clone().unwrap(), &point1, self.cr, false);
                    match point2 {
                        IntersectReturn::None => {
                            // TODO should I stream a null point here?
                            // stream.line_end(); ???
                            panic!("Must deal with no intersect.");
                        }
                        IntersectReturn::One(p) => {
                            match self.stream.clone() {
                                LineSinkEnum::CB(mut stream) => {
                                    stream.point(&p, Some(2));
                                    stream.line_end();
                                }
                                LineSinkEnum::CSE(stream) => {
                                    match stream {
                                        ClipSinkEnum::Blank => {
                                            panic!("ClickSinkEnum - actively using an unconnected blank");
                                        }
                                        ClipSinkEnum::Resample(mut stream) => {
                                            stream.point(&p, Some(2));
                                            stream.line_end();
                                        }
                                        ClipSinkEnum::Src(mut stream) => {
                                            stream.point(&p, Some(2));
                                            stream.line_end();
                                        }
                                    }
                                }
                            }
                            next = Some(p);
                        }
                        IntersectReturn::Two([_p, _]) => {
                            // self.stream.point(p, Some(2));
                            // self.stream.line_end();
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
                    let t = intersect(&point1, &self.point0.unwrap(), self.cr, true);
                    match t {
                        IntersectReturn::None => {}
                        IntersectReturn::One(_) => {
                            panic!("Requeted two received one.");
                        }
                        IntersectReturn::Two(t) => {
                            self.clean = CleanEnum::IntersectionsOrEmpty;
                            if self.small_radius {
                                match self.stream.clone() {
                                    LineSinkEnum::CB(mut stream) => {
                                        stream.line_start();
                                        stream.point(&t[0], None);
                                        stream.point(&t[1], None);
                                        stream.line_end();
                                    }
                                    LineSinkEnum::CSE(stream) => match stream {
                                        ClipSinkEnum::Blank => {
                                            panic!("ClickSinkEnum - actively using an unconnected blank");
                                        }
                                        ClipSinkEnum::Resample(mut stream) => {
                                            stream.line_start();
                                            stream.point(&t[0], None);
                                            stream.point(&t[1], None);
                                            stream.line_end();
                                        }
                                        ClipSinkEnum::Src(mut stream) => {
                                            stream.line_start();
                                            stream.point(&t[0], None);
                                            stream.point(&t[1], None);
                                            stream.line_end();
                                        }
                                    },
                                }
                            } else {
                                match self.stream.clone() {
                                    LineSinkEnum::CB(mut stream) => {
                                        stream.point(&t[1], None);
                                        stream.line_end();
                                        stream.line_start();
                                        stream.point(&t[0], Some(3u8));
                                    }
                                    LineSinkEnum::CSE(stream) => match stream {
                                        ClipSinkEnum::Blank => {
                                            panic!("ClickSinkEnum - actively using an unconnected blank");
                                        }
                                        ClipSinkEnum::Resample(mut stream) => {
                                            stream.point(&t[1], None);
                                            stream.line_end();
                                            stream.line_start();
                                            stream.point(&t[0], Some(3u8));
                                        }
                                        ClipSinkEnum::Src(mut stream) => {
                                            stream.point(&t[1], None);
                                            stream.line_end();
                                            stream.line_start();
                                            stream.point(&t[0], Some(3u8));
                                        }
                                    },
                                }
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
    }
    fn line_end(&mut self) {
        if self.v0 {
            match self.stream.clone() {
                LineSinkEnum::CB(mut stream) => {
                    stream.line_end();
                }
                LineSinkEnum::CSE(stream) => match stream {
                    ClipSinkEnum::Blank => {
                        panic!("ClickSinkEnum - actively using an unconnected blank");
                    }
                    ClipSinkEnum::Resample(mut stream) => {
                        stream.line_end();
                    }
                    ClipSinkEnum::Src(mut stream) => {
                        stream.line_end();
                    }
                },
            }
        }
        self.point0 = None;
    }
}
