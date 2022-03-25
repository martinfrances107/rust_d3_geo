/// Resample None.
pub mod none;
/// Resamples
pub mod resample;

use std::fmt::Debug;

/// Applied to both resampler strategeries :-
/// None and Resample.
pub trait Resampler: Clone + Debug {}
