use geo::CoordFloat;
use num_traits::FloatConst;

use crate::data_object::FeatureCollection;
use crate::stream::Stream;

use super::Streamable;

impl<T> Streamable for FeatureCollection<T>
where
    T: CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    fn to_stream<EP, SD>(&self, stream: &mut SD)
    where
        // EP: Stream<EP = EP, T = T> + Default,
        SD: Stream<EP = EP, T = T>,
    {
        for f in &self.0 {
            for g in &f.geometry {
                g.to_stream(stream);
            }
        }
    }
}
