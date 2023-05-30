use core::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;

use crate::clip::intersection::Intersection;

/// Given a vec of Intersetions :-
///
/// Set the next and pervious entries to be the elements above and
/// below. Connect the ends to form a circular loop.
pub(super) fn link<T>(array: &mut [Rc<RefCell<Intersection<T>>>])
where
    T: CoordFloat,
{
    if array.is_empty() {
        return;
    };
    let n = array.len();
    let mut a = array[0].clone();
    for elem in array.iter().take(n).skip(1) {
        let b = elem.clone();
        (*a).borrow_mut().n = Some(b.clone());
        (*b).borrow_mut().p = Some(a.clone());
        a = b;
    }
    let b = array[0].clone();
    (*a).borrow_mut().n = Some(b.clone());
    (*b).borrow_mut().p = Some(a);
}
