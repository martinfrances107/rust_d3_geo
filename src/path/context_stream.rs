use std::fmt::Display;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::area_stream::AreaStream;
use super::context::Context;
use super::string::String as PathString;
use super::Result;
use super::ResultEnum;

#[derive(Clone, Debug)]
pub enum ContextStream<T>
where
    T: CoordFloat,
{
    A(AreaStream<T>),
    C(Context<T>),
    S(PathString<T>),
    UNDEFINED,
}

impl<T> Default for ContextStream<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        ContextStream::UNDEFINED
    }
}

impl<T> Result for ContextStream<T>
where
    T: CoordFloat,
{
    type Out = Option<ResultEnum<T>>;
    fn result(&mut self) -> Self::Out {
        match self {
            ContextStream::A(a) => a.result(),
            ContextStream::C(pc) => pc.result(),
            ContextStream::S(ps) => ps.result(),
            ContextStream::UNDEFINED => panic!("Result of undefined."),
        }
    }
}

// impl<T> PointRadiusTrait for ContextStream<T>
// where
//     T: CoordFloat,
// {
//     type PrtT = Option<T>;
//     fn point_radius(&mut self, val: Self::PrtT) {
//         match self {
//             ContextStream::A(a) => a.point_radius(val),
//             ContextStream::C(c) => c.point_radius(val),
//             ContextStream::S(s) => s.point_radius(val),
//             ContextStream::UNDEFINED => panic!("radius of undefined."),
//         }
//     }
// }

impl<T> Stream for ContextStream<T>
where
    T: CoordFloat + Display + FloatConst,
{
    type T = T;

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self {
            ContextStream::A(a) => a.point(p, m),
            ContextStream::C(c) => c.point(p, m),
            ContextStream::S(s) => s.point(p, m),
            ContextStream::UNDEFINED => panic!("point of undefined."),
        }
    }
    fn sphere(&mut self) {
        match self {
            ContextStream::A(a) => a.sphere(),
            ContextStream::C(c) => c.sphere(),
            ContextStream::S(s) => s.sphere(),
            ContextStream::UNDEFINED => panic!("sphere of undefined."),
        }
    }
    fn line_start(&mut self) {
        match self {
            ContextStream::A(a) => a.line_start(),
            ContextStream::C(c) => c.line_start(),
            ContextStream::S(s) => s.line_start(),
            ContextStream::UNDEFINED => panic!("line_start of undefined."),
        }
    }
    fn line_end(&mut self) {
        match self {
            ContextStream::A(a) => a.line_end(),
            ContextStream::C(c) => c.line_end(),
            ContextStream::S(s) => s.line_end(),
            ContextStream::UNDEFINED => panic!("line_end of undefined."),
        }
    }
    fn polygon_start(&mut self) {
        match self {
            ContextStream::A(a) => a.polygon_start(),
            ContextStream::C(c) => c.polygon_start(),
            ContextStream::S(s) => s.polygon_start(),
            ContextStream::UNDEFINED => panic!("polygon start of undefined."),
        }
    }
    fn polygon_end(&mut self) {
        match self {
            ContextStream::A(a) => a.polygon_end(),
            ContextStream::C(c) => c.polygon_end(),
            ContextStream::S(s) => s.polygon_end(),
            ContextStream::UNDEFINED => panic!("polygon_end of undefined."),
        }
    }
}
