pub mod scale_translate_rotate;
pub mod st;
pub mod str;

use geo::CoordFloat;
use num_traits::FloatConst;

use self::str::Str;
use scale_translate_rotate::ScaleTranslateRotate;
use st::St;

pub fn generate<T>(k: &T, dx: &T, dy: &T, sx: &T, sy: &T, alpha: &T) -> ScaleTranslateRotate<T>
where
    T: CoordFloat + FloatConst,
{
    if alpha.is_zero() {
        ScaleTranslateRotate::ST(St {
            k: *k,
            dx: *dx,
            dy: *dy,
            sx: *sx,
            sy: *sy,
        })
    } else {
        let cos_alpha = alpha.cos();
        let sin_alpha = alpha.sin();
        ScaleTranslateRotate::STR(Str {
            a: cos_alpha * *k,
            b: sin_alpha * *k,
            ai: cos_alpha / *k,
            bi: sin_alpha / *k,
            ci: (sin_alpha * *dy - cos_alpha * *dx) / *k,
            fi: (sin_alpha * *dx + cos_alpha * *dy) / *k,
            dx: *dx,
            dy: *dy,
            sx: *sx,
            sy: *sy,
        })
    }
}
