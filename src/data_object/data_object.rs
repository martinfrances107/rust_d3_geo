use delaunator::Point;

/// The input data type use in D3
///  Can be special object ( DataObject )
///  or a vector of stuff
///  Null - here a blank.
#[derive(Clone, Debug)]
pub enum DataPrimitive {
    Vec(Vec<Point>),
}

#[derive(Clone, Debug)]
pub enum DataCollection {

    /// Feature - a feature containing one of the above geometry objects.
    Feature {
        feature: FeatureStruct,
    },


}

pub enum DataObject{
    Primitive{obj: DataPrimitive},
    Collection{collection: DataCollection},
    Blank,
}