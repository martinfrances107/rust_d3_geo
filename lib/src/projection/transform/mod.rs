/// Holds the scale rotate and translation output type.
pub mod scale_translate_rotate;
/// Scale Translate: One of the possible inner types.
pub mod st;
/// Scale Translate Rotate: One of the possible inner types.
pub mod str;

use geo::CoordFloat;

use self::str::Str;
use scale_translate_rotate::ScaleTranslateRotate;
use st::St;

/// Construct a `ScaleTranslateRotate` transform.
#[inline]
pub(crate) fn generate<T>(
    k: &T,
    dx: &T,
    dy: &T,
    sx: &T,
    sy: &T,
    alpha: &T,
) -> ScaleTranslateRotate<T>
where
    T: CoordFloat,
{
    if alpha.is_zero() {
        ScaleTranslateRotate::ST(St::new(k, dx, dy, sx, sy))
    } else {
        ScaleTranslateRotate::STR(Str::new(k, dx, dy, sx, sy, alpha))
    }
}
