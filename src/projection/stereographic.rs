use std::rc::Rc;

use delaunator::Point;

use super::projection::Projection;
use super::projection::StreamProcessorValueMaybe;
use super::projection_mutator::ProjectionMutator;
use crate::projection::azimuthal::azimuthal_invert;
use crate::Transform;

#[derive(Clone, Debug)]
pub struct StereographicRaw {}

impl StereographicRaw {
    fn new() -> Box<dyn Transform> {
        return Box::new(Self {});
    }

    pub fn gen_projection_mutator<'a>() -> ProjectionMutator {
        let s = Rc::new(StereographicRaw::new());
        let mut projection = ProjectionMutator::from_projection_raw(s);
        projection.scale(Some(&250f64));
        projection.clip_angle(StreamProcessorValueMaybe::Value(142f64));
        return projection;
    }
}

impl Transform for StereographicRaw {
    fn transform(&self, p: &Point) -> Point {
        let cy = p.y.cos();
        let k = 1f64 + p.x.cos() * cy;
        return Point {
            x: cy * p.x.sin() / k,
            y: p.y.sin() / k,
        };
    }

    fn invert(&self, p: &Point) -> Point {
        let f = Box::new(|z: f64| 2f64 * z.atan());
        let g = azimuthal_invert(f);
        return g(p.x, p.y);
    }
}
