#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

mod math;
mod clip;
mod projection;
mod resample;
mod rotation;
mod transform;
mod types;

pub mod compose;


/// Common to Projection, Rotation.
pub trait Transform<T> {
    fn transform(&self, p: &[T;2]) -> [T;2] {
        // Copy.
        return *p;
    }
    fn invert(&self, p: &[T;2]) -> [T;2] {
        return *p;
    }
}

struct TransformIdentity{}
impl<T> Transform<T> for TransformIdentity{}
