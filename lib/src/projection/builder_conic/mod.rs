use std::fmt::Debug;

use geo::CoordFloat;
use num_traits::FloatConst;

use super::{BuilderTrait, RawBase};

// mod angle;
// mod angle_get;
mod build;
mod center_get;
mod center_set;
// mod clip_angle_adjust;
// mod clip_angle_get;
// mod clip_angle_reset;
// mod clip_angle_set;
mod clip_extent_adjust;
// mod clip_extent_clear;
mod clip_extent_get;
// mod clip_extent_set;
mod fit;
// mod fit_no_clip;
// mod precision_adjust;
// mod precision_bypass;
// mod precision_get;
// mod precision_set;
mod parallels_get;
mod parallels_set;
// mod recenter_no_resampling;
// mod recenter_with_resampling;
// mod reflect_get;
// mod reflect_set;
// mod rotate_get;
mod rotate_set;
mod scale_get;
mod scale_set;
// mod transform;
mod translate_get;
mod translate_set;

// pub mod template;
// /// Builder shorthand notations.
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
pub struct Builder<BASE, PRConic, T>
where
    T: CoordFloat,
{
    base: BASE,
    /// Generates a raw projection.
    // pub pr: PR,
    /// The wrapped builder type.
    phi0: T,
    phi1: T,
    pr: PRConic,
}

/// Returns the pair of parallels used to define the projection.
pub trait ParallelsGet {
    /// f64 or f32.
    type T;

    /// Set the parallels.
    fn parallels(&mut self) -> (Self::T, Self::T);
}

/// Define the pair of parallels used to define the projection.
pub trait ParallelsSet {
    /// f64 or f32.
    type T;

    /// Set the parallels.
    fn parallels_set(&mut self, phi0: Self::T, phi1: Self::T) -> &mut Self;
}

impl<BASE, PR, T> BuilderTrait for Builder<BASE, PR, T>
where
    BASE: BuilderTrait<PR = PR>,
    PR: PRConic<T = T> + Clone,
    T: CoordFloat + Default + FloatConst,
{
    type PR = PR;

    /// Given a Raw Projection and a clipping defintion create the associated
    /// Projection builder.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as constants will always be converted into T.
    fn new(pr: PR) -> Self {
        let phi0 = T::zero();
        let phi1 = T::FRAC_PI_3();
        let pr = pr.generate(phi0, phi1);
        let base = BASE::new(pr.clone());
        Self {
            base,
            phi0,
            phi1,
            pr,
        }
    }

    fn update_pr(&mut self, pr: Self::PR) -> &mut Self {
        self.pr = pr.generate(self.phi0, self.phi1);
        self
    }
}
