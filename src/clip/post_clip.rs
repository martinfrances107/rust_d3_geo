use geo::CoordFloat;

use crate::clip::rectangle::rectangle::Rectangle;
use crate::identity::Identity;

/// A proto element for a stream pipeline stage.
#[derive(Clone, Debug)]
pub enum PostClip<T>
where
    T: CoordFloat,
{
    /// Pass thru.
    I(Identity),
    /// Clip Extent rectangle.
    R(Rectangle<T>),
}
