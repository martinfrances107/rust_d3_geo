mod rotation;
mod compose;

/// Common to Projection, Rotation.
pub trait Transform<T> {
    fn transform(&self, p: &[T; 2]) -> [T; 2];
    fn invert(&self, p: &[T; 2]) -> [T; 2];
}
