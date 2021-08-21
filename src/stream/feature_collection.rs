use geo::CoordFloat;
use num_traits::FloatConst;

use crate::data_object::FeatureCollection;
use crate::stream::Stream;

use super::Streamable;

impl<T> Streamable for FeatureCollection<T>
where
    T: CoordFloat + FloatConst,
{
    // type SD = Self;
    type T = T;
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        for f in &self.0 {
            for g in &f.geometry {
                g.to_stream(stream);
            }
        }
    }
}
