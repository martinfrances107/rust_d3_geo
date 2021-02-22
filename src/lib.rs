// #![allow(clippy::needless_return)]
#![allow(clippy::all)]
// #![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]

use geo::{CoordFloat, Coordinate};
use std::marker::PhantomData;

extern crate web_sys;
pub mod cartesian;
pub mod centroid;
pub mod circle;
pub mod data_object;
pub mod distance;
pub mod in_delta;
pub mod length;
pub mod path;
pub mod polygon_contains;
pub mod projection;
pub mod rotation;

mod clip;
mod compose;
mod point_equal;
mod stream;
// mod transform_stream;

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Debug, Default)]
pub struct TransformIdentity<T>
where
    T: CoordFloat,
    T: std::default::Default,
{
    phantom: PhantomData<T>,
}

impl<T: CoordFloat + std::default::Default> Transform for TransformIdentity<T> {
    type C = Coordinate<T>;
}

// Common to Projection, Rotation.
pub trait Transform {
    type C: Clone;
    #[inline]
    fn transform(&self, p: &Self::C) -> Self::C {
        p.clone()
    }

    #[inline]
    fn invert(&self, p: &Self::C) -> Self::C {
        p.clone()
    }
}
