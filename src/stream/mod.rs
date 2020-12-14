pub(crate) mod geometry_processor;
mod line;
mod polygon;

pub trait Stream<T> {
    fn point(&mut self, _x: T, _y: T, _z: Option<T>) {}
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
}
