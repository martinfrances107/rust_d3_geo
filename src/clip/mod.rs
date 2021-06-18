pub mod antimeridian;
pub mod circle;
pub mod clip;
pub mod clip_base;
pub mod clip_buffer;
pub mod clip_raw;
pub mod clip_sink_enum;
pub mod compare_intersections;
pub mod interpolate_trait;
pub mod line_elem;
pub mod line_enum;
pub mod line_sink_enum;
pub mod line_trait;
pub mod point_visible_trait;
pub mod rejoin;

// use std::cell::RefCell;
// use std::cmp::Ordering;
// use std::fmt::Display;
// use std::ops::AddAssign;
// use std::rc::Rc;

// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::Float;
// use num_traits::FloatConst;

// use crate::stream::Stream;

use clip_buffer::ClipBuffer;
// use rejoin::intersection::Intersection;

// use crate::projection::ProjectionRawTrait;
// use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
// use interpolate_trait::Interpolate;
use crate::stream::stream_in_trait::StreamIn;
use point_visible_trait::PointVisible;
// pub trait Clip: PointVisible + Interpolate + Stream

// Takes a line and cuts into visible segments. Return values used for polygon
// clipPing: 0 - there were intersections or the line was empty; 1 - no
// intersections 2 - there were intersections, and the first and last segments
// should be rejoined.
#[derive(Debug, Clone, Copy)]
pub enum CleanEnum {
    Undefined,
    IntersectionsOrEmpty,
    NoIntersections,
    IntersectionsRejoin,
}

impl Default for CleanEnum {
    fn default() -> CleanEnum {
        CleanEnum::Undefined
    }
}

pub trait Clean {
    /// A clip trait.
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    fn clean(&self) -> CleanEnum;
}

pub trait Clip: PointVisible + Stream
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + Float + FloatConst,
{
    // type C;
}

// // Line wrapped around a clip buffer.
/// TODO must revisit SInput always equal ClipBuffer<T> here
/// is there a way to do static dispatch here!!!
pub trait LCB: Clean + Stream + StreamIn {}
