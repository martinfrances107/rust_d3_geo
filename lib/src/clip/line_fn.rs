use geo::CoordFloat;

pub(super) fn line<T>(
    a: &mut [T; 2],
    b: &mut [T; 2],
    x0: T,
    y0: T,
    x1: T,
    y1: T,
) -> bool
where
    T: CoordFloat,
{
    let ax = a[0];
    let ay = a[1];
    let bx = b[0];
    let by = b[1];
    let dx = bx - ax;
    let dy = by - ay;

    let mut t0 = T::zero();
    let mut t1 = T::one();
    let mut r = x0 - ax;

    if dx.is_zero() && r > T::zero() {
        return false;
    }
    r = r / dx;
    if dx < T::zero() {
        if r < t0 {
            return false;
        }
        if r < t1 {
            t1 = r;
        }
    } else if dx > T::zero() {
        if r > t1 {
            return false;
        }
        if r > t0 {
            t0 = r;
        }
    }

    r = x1 - ax;
    if dx.is_zero() && r < T::zero() {
        return false;
    }
    r = r / dx;
    if dx < T::zero() {
        if r > t1 {
            return false;
        }
        if r > t0 {
            t0 = r;
        }
    } else if dx > T::zero() {
        if r < t0 {
            return false;
        }
        if r < t1 {
            t1 = r;
        }
    }

    r = y0 - ay;
    if dy.is_zero() && r > T::zero() {
        return false;
    }
    r = r / dy;
    if dy < T::zero() {
        if r < t0 {
            return false;
        }
        if r < t1 {
            t1 = r;
        }
    } else if dy > T::zero() {
        if r > t1 {
            return false;
        }
        if r > t0 {
            t0 = r;
        }
    }

    r = y1 - ay;
    if dy.is_zero() && r < T::zero() {
        return false;
    }
    r = r / dy;
    if dy < T::zero() {
        if r > t1 {
            return false;
        }
        if r > t0 {
            t0 = r;
        }
    } else if dy > T::zero() {
        if r < t0 {
            return false;
        }
        if r < t1 {
            t1 = r;
        }
    }

    if t0 > T::zero() {
        a[0] = ax + t0 * dx;
        a[1] = ay + t0 * dy;
    }
    if t1 < T::one() {
        b[0] = ax + t1 * dx;
        b[1] = ay + t1 * dy;
    }
    true
}
