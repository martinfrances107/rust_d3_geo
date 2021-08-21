// fn stream_geometry_fn<DRAIN>(geometry: Option<Geometry>, stream: Rc<RefCell<DRAIN>>) {
//     match
// }

// fn stream_fn(object: DataObject, stream: Rc<RefCell<DRAIN>>>) {
//     match object{
//         DataObject::FeatureCollection(fc){
//             for f in fc.features{
//                 stream_geometry(f.geometry, stream);
//             }
//         }
//         DataObject::Feature(f) {
//             stream_geometry(f.geometry, stream);
//         }
//         _ => {
//             stream_geometry(object, stream);
//         }
//     }

// }
