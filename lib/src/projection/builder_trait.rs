use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::Transform;

use super::Center;
use super::ClipAngle;
use super::ClipExtent;
use super::Scale;
use super::Translate;

/// Projection builder Trait.
pub trait BuilderTrait: Center + ClipAngle + ClipExtent + Scale + Translate
where
    <Self as BuilderTrait>::PR: Transform<T = <Self as BuilderTrait>::T>,
    <Self as BuilderTrait>::T: AddAssign + AsPrimitive<<Self as BuilderTrait>::T> + CoordFloat,
{
    /// Projection Raw.
    type PR;
    /// f64 or f32.
    type T;

    /// Returns the projection’s current resampling precision which defaults to square root of 0.5.
    /// This value corresponds to the Douglas–Peucker distance.
    ///
    fn get_precision(self) -> <Self as BuilderTrait>::T;

    ///  Sets the threshold for the projection’s adaptive resampling to the specified value in PIxels and returns the projection.
    ///  This value corresponds to the Douglas–Peucker distance.
    ///
    ///  @param precision A numeric value in PIxels to use as the threshold for the projection’s adaptive resampling.
    ///
    fn precision(self, delta: &<Self as BuilderTrait>::T) -> Self
    where
        <Self as BuilderTrait>::PR: Transform<T = <Self as BuilderTrait>::T>,
        <Self as BuilderTrait>::T: AddAssign
            + AsPrimitive<<Self as BuilderTrait>::T>
            + CoordFloat
            + Debug
            + Display
            + FloatConst;

    ///  Returns the projection’s current angle, which defaults to 0°.
    ///
    /// angle(): number;
    ///
    ///   Sets the projection’s post-projection planar rotation angle to the specified angle in degrees and returns the projection.
    ///   @param angle The new rotation angle of the projection.
    ///
    fn reset(self) -> Self
    where
        <Self as BuilderTrait>::PR: Transform<T = <Self as BuilderTrait>::T>,
        <Self as BuilderTrait>::T: AddAssign
            + AsPrimitive<<Self as BuilderTrait>::T>
            + CoordFloat
            + Debug
            + Display
            + FloatConst;

    /// Adjust the translate portion of the scale translate rotate tranform.
    fn recenter(self) -> Self
    where
        <Self as BuilderTrait>::PR: Transform<T = <Self as BuilderTrait>::T>,
        <Self as BuilderTrait>::T: AddAssign
            + AsPrimitive<<Self as BuilderTrait>::T>
            + CoordFloat
            + Debug
            + Display
            + FloatConst;

    /// Returns the projection builders three-axis rotation.
    fn get_rotate(&self) -> [<Self as BuilderTrait>::T; 3]
    where
        <Self as BuilderTrait>::PR: Transform<T = <Self as BuilderTrait>::T>,
        <Self as BuilderTrait>::T: AddAssign
            + AsPrimitive<<Self as BuilderTrait>::T>
            + CoordFloat
            + Debug
            + Display
            + FloatConst;

    ///   Sets the projection builder’s three-axis rotation to the specified angles, which must be a two- or three-element array of numbers.
    ///
    ///   @param angles  A two- or three-element array of numbers [lambda, phi, gamma] specifying the rotation angles in degrees about each spherical axis.
    ///   (These correspond to yaw, PItch and roll.) If the rotation angle gamma is omitted, it defaults to 0.
    fn rotate(self, angles: [<Self as BuilderTrait>::T; 3]) -> Self
    where
        <Self as BuilderTrait>::PR: Transform<T = <Self as BuilderTrait>::T>,
        <Self as BuilderTrait>::T: AddAssign
            + AsPrimitive<<Self as BuilderTrait>::T>
            + CoordFloat
            + Debug
            + Display
            + FloatConst;
}
