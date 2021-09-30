use geo::CoordFloat;

use crate::clip::rectangle::rectangle::Rectangle;
use crate::identity::Identity;

#[derive(Clone, Debug)]
pub enum PostClip<T>
where
    T: CoordFloat,
{
    I(Identity),
    R(Rectangle<T>),
}
