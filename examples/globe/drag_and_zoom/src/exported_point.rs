use wasm_bindgen::prelude::wasm_bindgen;

/// Used to export a `Coord<f64>` to javascript.
#[derive(Debug)]
#[wasm_bindgen]
pub struct ExportedPoint {
    /// x coordinate.
    pub x: f64,
    /// y coordinate.
    pub y: f64,
}

#[wasm_bindgen]
#[allow(clippy::missing_const_for_fn)]
impl ExportedPoint {
    /// Constructor.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Getter
    #[must_use]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// Getter
    #[must_use]
    pub fn get_y(&self) -> f64 {
        self.y
    }
}
