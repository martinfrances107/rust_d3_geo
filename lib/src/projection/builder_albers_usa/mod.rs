use std::fmt::Debug;
use std::marker::PhantomData;

use super::albers_usa::AlbersUsa;
use super::BuilderTrait;
use super::RawBase;

// mod angle;
// mod angle_get;
// mod build;
// mod center_get;
// mod center_set;
// mod clip_angle_adjust;
// mod clip_angle_get;
// mod clip_angle_reset;
// mod clip_angle_set;
// mod clip_extent_adjust;
// mod clip_extent_clear;
// mod clip_extent_get;
mod clip_extent_set;
// mod fit;
// mod fit_no_clip;
// mod precision_adjust;
// mod precision_bypass;
// mod precision_get;
// mod precision_set;
// mod parallels_get;
// mod parallels_set;
// mod recenter_no_resampling;
// mod recenter_with_resampling;
// mod reflect_get;
// mod reflect_set;
// mod rotate_get;
// mod rotate_set;
// mod scale_get;
// mod scale_set;
// mod transform;
mod translate_get;
mod translate_set;

// pub mod template;
/// Builder shorthand notations.
pub mod types;

/// Adjustments the pair of parallels
/// use to define the proejctions.
///
/// Differs from PR in the way the PR is generated.
pub trait PRConic: RawBase {
    /// Late initisalisation of a projection
    /// based on a pair of parallels.
    #[must_use]
    fn generate(self, y0: Self::T, y1: Self::T) -> Self;
}

/// A wrapper over Projection\Builder which hold state phi0, phi1 and allow regeneration of the PR.
#[derive(Clone, Debug)]
pub struct Builder<DRAIN>
where
    DRAIN: Clone,
    //     T: CoordFloat,
{
    phantom_drain: PhantomData<DRAIN>,
    pub pr: AlbersUsa<DRAIN>,
}

impl<DRAIN> BuilderTrait for Builder<DRAIN>
where
    DRAIN: Clone,
{
    type PR = AlbersUsa<DRAIN>;

    /// Given a Raw Projection and a clipping defintion create the associated
    /// Projection builder.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as constants will always be converted into T.
    fn new(pr: AlbersUsa<DRAIN>) -> Self {
        Self {
            phantom_drain: PhantomData::<DRAIN>,
            pr,
        }
    }
}
