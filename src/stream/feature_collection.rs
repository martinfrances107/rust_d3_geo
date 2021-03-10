use super::Streamable;
// use crate::data_object::FeaturesStruct;
use crate::data_object::FeatureCollection;
use crate::stream::Stream;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

impl<T: CoordFloat + FloatConst> Streamable for FeatureCollection<T> {
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut Box<dyn Stream<C = Self::SC>>) {
        for f in &self.0 {
            for g in &f.geometry {
                g.to_stream(stream);
            }
        }
    }
}
