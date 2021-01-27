use super::Streamable;
// use crate::data_object::FeaturesStruct;
use crate::data_object::FeatureCollection;
use crate::stream::Stream;
use geo::CoordFloat;
use num_traits::FloatConst;

impl<T: CoordFloat + FloatConst> Streamable<T> for FeatureCollection<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for f in &self.0 {
            for g in &f.geometry {
                g.to_stream(stream);
            }
        }
    }
}
