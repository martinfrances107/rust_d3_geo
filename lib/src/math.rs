use geo::CoordFloat;
use num_traits::FloatConst;

/// Floating point number within this range are considered indistinguiable.
pub const EPSILON: f64 = 1e-6;
/// The precision used to distinguish floating point numbers squared.
pub const EPSILON2: f64 = 1e-12;

/// RUST and JS verison of this funcion are identical.
/// Here outside the range -1 < x < 1 we return +/- PI/2
/// as opposed to NAN.
pub fn acos<T>(x: T) -> T
where
    T: CoordFloat + FloatConst,
{
    if x > T::one() {
        T::zero()
    } else if x < T::from(-1_f64).unwrap() {
        T::PI()
    } else {
        x.acos()
    }
}

/// RUST and JS verison of this funcion are identical.
/// Here outside the range -1 < x < 1 we return +/- PI/2
/// as opposed to NAN.
pub fn asin<T>(x: T) -> T
where
    T: CoordFloat + FloatConst,
{
    if x > T::one() {
        T::FRAC_PI_2()
    } else if x < T::from(-1_f64).unwrap() {
        -T::FRAC_PI_2()
    } else {
        x.asin()
    }
}
