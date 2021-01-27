use super::Streamable;
use crate::stream::Stream;
use geo::{CoordFloat, GeometryCollection};

impl<T: CoordFloat + num_traits::FloatConst> Streamable<T> for GeometryCollection<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for g in self {            
            g.to_stream(stream);
        }
    }
}
