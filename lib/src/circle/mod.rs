#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

/// A circle related helper function.
pub mod calc_radius;
/// Generator use to inject circles into a stream.
pub mod generator;
/// Holds the output type of the generator.
pub mod stream;
/// Helper function.
pub mod stream_fn;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
