use std::rc::Rc;

use delaunator::Point;

use super::projection::Projection;
use super::projection::StreamProcessorValueMaybe;
use super::projection_mutator::ProjectionMutator;
use crate::projection::azimuthal::azimuthal_invert;
use crate::Transform;

#[derive(Clone, Debug)]
pub struct OrthographicRaw {}

impl OrthographicRaw {
    fn new() -> Box<dyn Transform> {
        return Box::new(Self {});
    }

    pub fn gen_projection_mutator<'a>() -> ProjectionMutator {
        let s = Rc::new(OrthographicRaw::new());
        let mut projection = ProjectionMutator::from_projection_raw(s);
        projection.scale(Some(&249.5f64));
        let angle = 249.5f64;
        projection.clip_angle(StreamProcessorValueMaybe::Value(angle));
        return projection;
    }
}

impl Transform for OrthographicRaw {
    fn transform(&self, p: &Point) -> Point {
        return Point {
            x: p.y.cos() * p.x.sin(),
            y: p.y.sin(),
        };
    }

    fn invert(&self, p: &Point) -> Point {
        let f = Box::new(|z: f64| z.asin());
        let g = azimuthal_invert(f);
        return g(p.x, p.y);
    }
}
