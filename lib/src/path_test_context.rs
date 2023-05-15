/// Used by `index_test`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path2d {
    buffer: Vec<String>,
}

/// Mocks of Path2d
///
/// Returns "Result" to be compatilble
/// with the browsers `Path2D::new()`.
#[allow(clippy::unnecessary_wraps)]
impl Path2d {
    /// Buffered strings.
    pub const fn new() -> Result<Self, ()> {
        Ok(Self { buffer: vec![] })
    }
}

impl Path2d {
    /// Buffered strings.
    pub fn result(&mut self) -> Vec<String> {
        let result = self.buffer.clone();
        self.buffer = vec![];
        result
    }
}

impl Path2d {
    #[inline]
    #[allow(clippy::unnecessary_wraps)]
    /// Shadows methods in the browser.
    pub fn arc(&mut self, x: f64, y: f64, r: f64, _start: f64, _stop: f64) -> Result<(), bool> {
        self.buffer.push(format!(
            "type: arc, x: {:?}, y: {:?}, r: {:?}",
            x.round(),
            y.round(),
            r
        ));
        Ok(())
    }

    #[inline]
    /// Shadows methods in the browser.
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.buffer.push(format!(
            "type: moveTo, x: {:?}, y: {:?}",
            x.round(),
            y.round()
        ));
    }

    #[inline]
    /// Shadows methods in the browser.
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.buffer.push(format!(
            "type: lineTo, x: {:?}, y: {:?}",
            x.round(),
            y.round()
        ));
    }

    #[inline]
    /// Shadows methods in the browser.
    pub fn close_path(&mut self) {
        self.buffer.push("closePath".to_string());
    }
}
