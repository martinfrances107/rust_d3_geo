use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::path::PointRadiusTrait;
use crate::stream::Stream;

use super::area::Area;
use super::bounds::Bounds;
use super::centroid::Centroid;
use super::context::Context;
use super::string::String as PathString;
use super::Result;
use super::ResultEnum;

/// Context Stream which stream endpoint is being considered.
/// TODO can I optimise this away.
#[derive(Clone, Debug)]
pub enum ContextStream<T>
where
    T: CoordFloat,
{
    /// Path area endpoint.
    A(Area<T>),
    /// Bounds endpoint.
    B(Bounds<T>),
    /// Path context endpoint.
    Context(Context<T>),
    /// Path centroid endpoint.
    Centroid(Centroid<T>),
    /// Path string endpoint.
    S(PathString<T>),
    /// Uninitialised state.
    UNDEFINED,
}

/// This is requires when building a Projection
/// when the final ContextStram is not yet defined.
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
    T: AddAssign<T> + CoordFloat,
{
    type Out = Option<ResultEnum<T>>;
    fn result(&mut self) -> Self::Out {
        match self {
            // ContextStream::A(a) => a.result(),
            ContextStream::A(pc) => pc.result(),
            ContextStream::B(pc) => pc.result(),
            ContextStream::Centroid(pc) => pc.result(),
            ContextStream::Context(pc) => pc.result(),
            ContextStream::S(ps) => ps.result(),
            ContextStream::UNDEFINED => panic!("Result of undefined."),
        }
    }
}

impl<T> PointRadiusTrait for ContextStream<T>
where
    T: CoordFloat,
{
    type PrtT = T;
    fn point_radius(&mut self, val: Self::PrtT) {
        match self {
            ContextStream::A(_a) => todo!("how to handle this?"),
            ContextStream::B(_b) => todo!("how to handle this?"),
            ContextStream::Centroid(_c) => todo!("how to handle this?"),
            ContextStream::Context(c) => c.point_radius(val),
            ContextStream::S(s) => s.point_radius(val),
            ContextStream::UNDEFINED => panic!("radius of undefined."),
        }
    }
}

impl<T> Stream for ContextStream<T>
where
    T: AddAssign<T> + CoordFloat + Display + FloatConst,
{
    type T = T;
    type EP = Self;

    #[inline]
    fn get_endpoint(self) -> Self {
        self
    }

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self {
            ContextStream::A(a) => a.point(p, m),
            ContextStream::B(b) => b.point(p, m),
            ContextStream::Centroid(c) => c.point(p, m),
            ContextStream::Context(c) => c.point(p, m),
            ContextStream::S(s) => s.point(p, m),
            ContextStream::UNDEFINED => panic!("point of undefined."),
        }
    }
    fn sphere(&mut self) {
        match self {
            ContextStream::A(a) => a.sphere(),
            ContextStream::B(b) => b.sphere(),
            ContextStream::Centroid(c) => c.sphere(),
            ContextStream::Context(c) => c.sphere(),
            ContextStream::S(s) => s.sphere(),
            ContextStream::UNDEFINED => panic!("sphere of undefined."),
        }
    }
    fn line_start(&mut self) {
        match self {
            ContextStream::A(a) => a.line_start(),
            ContextStream::B(b) => b.line_start(),
            ContextStream::Centroid(c) => c.line_start(),
            ContextStream::Context(c) => c.line_start(),
            ContextStream::S(s) => s.line_start(),
            ContextStream::UNDEFINED => panic!("line_start of undefined."),
        }
    }
    fn line_end(&mut self) {
        match self {
            ContextStream::A(a) => a.line_end(),
            ContextStream::B(b) => b.line_end(),
            ContextStream::Centroid(c) => c.line_end(),
            ContextStream::Context(c) => c.line_end(),
            ContextStream::S(s) => s.line_end(),
            ContextStream::UNDEFINED => panic!("line_end of undefined."),
        }
    }
    fn polygon_start(&mut self) {
        match self {
            ContextStream::A(a) => a.polygon_start(),
            ContextStream::B(b) => b.polygon_start(),
            ContextStream::Centroid(c) => c.polygon_start(),
            ContextStream::Context(c) => c.polygon_start(),
            ContextStream::S(s) => s.polygon_start(),
            ContextStream::UNDEFINED => panic!("polygon start of undefined."),
        }
    }
    fn polygon_end(&mut self) {
        match self {
            ContextStream::A(a) => a.polygon_end(),
            ContextStream::B(b) => b.polygon_end(),
            ContextStream::Centroid(c) => c.polygon_end(),
            ContextStream::Context(c) => c.polygon_end(),
            ContextStream::S(s) => s.polygon_end(),
            ContextStream::UNDEFINED => panic!("polygon_end of undefined."),
        }
    }
}
