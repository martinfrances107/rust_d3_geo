use std::fmt::Display;
use std::ops::AddAssign;
// use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::intersect::intersect;
use super::intersect::IntersectReturn;

// use crate::clip::clip_buffer::ClipBuffer;
// use crate::clip::clip_sink_enum::ClipSinkEnum;
use crate::clip::line_elem::LineElem;
// use crate::clip::line_sink_enum::LineSinkEnum;
use crate::clip::clip_buffer::ClipBuffer;
use crate::clip::LCB;
use crate::point_equal::point_equal;
// use crate::projection::ProjectionRawTrait;
// use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
// use crate::stream::StreamSourceDummy;
use crate::clip::Clean;
use crate::clip::CleanEnum;
use crate::stream::stream_in_trait::StreamIn;
// use crate::Transform;

// #[derive(Debug)]
pub struct Line<STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    c0: u8,           // code for previous point
    clean: CleanEnum, // no intersections
    radius: T,
    cr: T,
    not_hemisphere: bool,
    point0: Option<LineElem<T>>, // previous point
    small_radius: bool,
    stream: STREAM,
    v0: bool,  // visibility of previous point
    v00: bool, // visibility of first point
}

// impl<'a, PR, SD, T> Default for Line<'a, PR, SD, T>
// where
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     PR: Transform<C = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     fn default() -> Self {
//         Self {
//             c0: 0u8,
//             clean: CleanEnum::Undefined,
//             radius: T::zero(),
//             cr: T::zero(),
//             not_hemisphere: false,
//             point0: None,
//             small_radius: false,
//             stream: LineSinkEnum::CSE(ClipSinkEnum::Src(StreamDst::SRC(
//                 StreamSourceDummy::default(),
//             ))),
//             v0: false,
//             v00: false,
//         }
//     }
// }

impl<STREAM, T> Line<STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
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
            stream: STREAM::default(),
        }
    }

    #[inline]
    fn visible(&self, p: &Coordinate<T>) -> bool {
        println!("point_visible cr {:?}", self.cr);
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

impl<T> LCB for Line<ClipBuffer<T>, T> where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst
{
}

impl<STREAM, T> StreamIn for Line<STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SInput = STREAM;
    #[inline]
    fn stream_in(&mut self, stream: STREAM) {
        self.stream = stream;
    }

    // #[inline]
    // pub fn get_stream(&mut self) -> &mut STREAM {
    //     &mut self.stream
    // }
}

impl<STREAM, T> Clean for Line<STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    #[inline]
    fn clean(&self) -> CleanEnum {
        println!("line(c) clean() {:#?} {} {}", self.clean, self.v00, self.v0);
        if self.v00 && self.v0 {
            println!("output = rejoin");
            CleanEnum::IntersectionsRejoin
        } else {
            self.clean
        }
    }
}

impl<STREAM, T> Stream for Line<STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    // type ST = T;
    type SC = Coordinate<T>;
    // type SD = SD;

    fn sphere(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}

    // fn get_dst(
    //     &self,
    // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
    // {
    //     self.stream.get_dst()
    // }
    fn line_start(&mut self) {
        self.v00 = false;
        self.v0 = false;
        self.clean = CleanEnum::NoIntersections;
    }

    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        let point1 = Some(LineElem { p: *p, m: None });
        let mut point2: Option<LineElem<T>>;
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
        println!("clip:circle point entry v, c {:?} {:?}", v, c);
        println!("clip:circle point self.point0 {:?}", self.point0);
        if self.point0.is_none() {
            self.v00 = v;
            self.v0 = v;
            println!("setting v ");
            if v {
                self.stream.line_start();
            }
        }
        println!("before intersect check {:?} {:?}", v, self.v0);
        if v != self.v0 {
            println!("intersect check");
            point2 = match intersect(
                &self.point0.clone().unwrap(),
                &point1.clone().unwrap(),
                self.radius.cos(),
                false,
            ) {
                IntersectReturn::One(p_return) => p_return,
                IntersectReturn::None => None,
                IntersectReturn::False => {
                    todo!("This case is not handled by test");
                    // I think I should set point2 to None here buy must test.
                }
                IntersectReturn::Two(_t) => {
                    // There is a subtle bug in the javascript here two points is handles
                    // as if the second does not exits.
                    // For now just cause a panic here to see how many times it occurs.
                    panic!("Requested One or None found Two as !!");
                }
            };
            if point2.is_some()
                || point_equal(self.point0.clone().unwrap().p, point2.unwrap().p.clone())
                || point_equal(point1.unwrap().p, point2.unwrap().p)
            {
                point1.unwrap().m = Some(1u8);
            }
        }

        if v != self.v0 {
            self.clean = CleanEnum::IntersectionsOrEmpty;
            if v {
                println!("outside going in");
                // outside going in
                self.stream.line_start();
                point2 = match intersect(
                    &point1.clone().unwrap(),
                    &self.point0.clone().unwrap(),
                    self.cr,
                    false,
                ) {
                    IntersectReturn::One(le) => le,
                    IntersectReturn::Two([_p, _m]) => {
                        panic!("Silently dropping second point.");
                    }
                    IntersectReturn::None => None,
                    IntersectReturn::False => {
                        todo!("must cover this case.");
                    }
                };
                self.stream.point(&point2.unwrap().p, None)
            } else {
                // inside going out
                println!("inside going out");
                point2 = match intersect(
                    &self.point0.clone().unwrap(),
                    &point1.clone().unwrap(),
                    self.cr,
                    false,
                ) {
                    IntersectReturn::One(le) => le,
                    IntersectReturn::Two([_, _]) => {
                        panic!("Silently dropping second point.");
                    }
                    IntersectReturn::None => None,
                    IntersectReturn::False => {
                        todo!("must handle this case.");
                    }
                };
                // panic!("about to insert 2 ");
                self.stream.point(&point2.unwrap().p, Some(2));
                self.stream.line_end();
            }
            self.point0 = point2;
        } else if self.not_hemisphere && self.point0.is_some() && self.small_radius ^ v {
            println!("within a small circle");
            // If the codes for two points are different, or are both zero,
            // and there this segment intersects with the small circle.
            if self.c0 != c || c == 0 {
                let t = intersect(
                    &point1.clone().unwrap(),
                    &self.point0.unwrap(),
                    self.cr,
                    true,
                );
                match t {
                    IntersectReturn::False => {
                        // None found
                    }
                    IntersectReturn::None => {
                        // do nothing.
                    }
                    IntersectReturn::One(_) => {
                        panic!("Requeted two received one or none.");
                    }
                    IntersectReturn::Two(t) => {
                        self.clean = CleanEnum::IntersectionsOrEmpty;
                        if self.small_radius {
                            println!("small radius");
                            self.stream.line_start();
                            self.stream.point(&t[0], None);
                            self.stream.point(&t[1], None);
                            self.stream.line_end();
                        } else {
                            println!("not a small radius");
                            self.stream.point(&t[1], None);
                            self.stream.line_end();
                            self.stream.line_start();
                            self.stream.point(&t[0], Some(3u8));
                        }
                    }
                }
            }
        }
        println!("v, !point0 {:?} {:?}", v, self.point0.is_none());
        if v && (self.point0.is_none() || !point_equal(self.point0.unwrap().p, point1.unwrap().p)) {
            println!("point equal pass");
            self.stream.point(&point1.unwrap().p, None);
        }
        self.point0 = point1;
        self.v0 = v;
        self.c0 = c;
        println!("point end()");
    }
    fn line_end(&mut self) {
        println!("clip circle line_end()");
        if self.v0 {
            self.stream.line_end();
        }
        self.point0 = None;
    }
}
