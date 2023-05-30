use core::fmt::Debug;

use geo::CoordFloat;
use num_traits::FloatConst;

use super::BuilderTrait;
use super::RawBase;
use super::ScaleGet;

mod build;
mod center_get;
mod center_set;
mod clip_angle_set;
mod clip_extent_adjust;
mod clip_extent_get;
mod clip_extent_set;
mod fit;
mod parallels_get;
mod parallels_set;
mod rotate_set;
mod scale_get;
mod scale_set;
mod translate_get;
mod translate_set;

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

/// A wrapper over a projection builder which holds state about the pair of
/// parallels.
#[derive(Clone, Debug)]
pub struct Builder<BASE, T>
where
    BASE: ScaleGet<T = T>,
    T: CoordFloat,
{
    base: BASE,
    phi0: T,
    phi1: T,
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

impl<BASE, PR, T> BuilderTrait for Builder<BASE, T>
where
    BASE: BuilderTrait<PR = PR> + ScaleGet<T = T>,
    PR: PRConic<T = T>,
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
        let base = BASE::new(pr);
        Self { base, phi0, phi1 }
    }
}
