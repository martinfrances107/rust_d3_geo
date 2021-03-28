// #![allow(clippy::needless_return)]
#![allow(clippy::all)]
// #![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]

use geo::{CoordFloat, Coordinate};
use std::marker::PhantomData;

extern crate derivative;
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
mod constant;
mod point_equal;
mod stream;
// mod transform_stream;

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct TransformIdentity<T>
where
    T: CoordFloat + Default,
{
    pub phantom: PhantomData<T>,
}

// impl<'a, T: CoordFloat + Default> TransformClone<'a> for TransformIdentity<T> {
//     fn box_clone(&'a self) -> Box<dyn TransformClone<'a, TcC = Self::TcC>> {
//         Box::new(self.clone())
//     }
// }

impl<T: CoordFloat + Default> Transform for TransformIdentity<T> {
    type TcC = Coordinate<T>;
    fn transform(&self, p: &Self::TcC) -> Self::TcC {
        *p
    }
    fn invert(&self, p: &Self::TcC) -> Self::TcC {
        *p
    }
}

pub trait TransformClone<'a>: Transform {
    fn box_clone(&'a self) -> Box<dyn TransformClone<'a, TcC = Self::TcC>>;
}

// Common to Projection, Rotation.
pub trait Transform {
    type TcC;
    fn transform(&self, p: &Self::TcC) -> Self::TcC;
    fn invert(&self, p: &Self::TcC) -> Self::TcC;
}
