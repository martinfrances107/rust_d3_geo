use geo::CoordFloat;
use geo::Coordinate;

#[cfg(not(tarpaulin_include))]
/// Used by index_test
#[derive(Clone, Debug, Default)]
pub struct TestContext {
    buffer: Vec<String>,
}

#[cfg(not(tarpaulin_include))]
/// Stub for web_sys rendering context.
pub trait RenderingContext2d<T>
where
    T: CoordFloat,
{
    fn arc(&mut self, p: &Coordinate<T>, r: T, start: T, stop: T);
    fn close_path(&mut self);
    fn line_to(&mut self, p: &Coordinate<T>);
    fn move_to(&mut self, p: &Coordinate<T>);
    fn rect(&mut self, p: &Coordinate<T>, w: T, h: T);
}

#[cfg(not(tarpaulin_include))]
impl TestContext {
    /// Buffered strings.
    pub fn result(&mut self) -> Vec<String> {
        let result = self.buffer.clone();
        self.buffer = vec![];
        result
    }
}

#[cfg(not(tarpaulin_include))]
impl<T> RenderingContext2d<T> for TestContext
where
    T: CoordFloat,
{
    fn arc(&mut self, p: &Coordinate<T>, r: T, _start: T, _stop: T) {
        {
            self.buffer.push(format!(
                "type: arc, x: {:?}, y: {:?}, r: {:?}",
                p.x.round(),
                p.y.round(),
                r
            ));
        }
    }
    fn move_to(&mut self, p: &Coordinate<T>) {
        self.buffer
            .push(format!("type: moveTo {:?}, {:?}", p.x.round(), p.y.round()));
    }
    fn line_to(&mut self, p: &Coordinate<T>) {
        self.buffer
            .push(format!("type: lineTo {:?}, {:?}", p.x.round(), p.y.round()));
    }
    fn close_path(&mut self) {
        self.buffer.push(format!("closePath"));
    }
    fn rect(&mut self, p: &Coordinate<T>, w: T, h: T) {
        todo!("test do not use this.");
    }
}
