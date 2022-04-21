use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::intersection::Intersection;

/// Given a vec of Intersetions :-
///
/// Set the next and pervious entries to be the elements above and
/// below. Connect the ends to form a circular loop.
pub fn link<T>(array: &mut [Rc<RefCell<Intersection<T>>>])
where
    T: CoordFloat + FloatConst,
{
    if array.is_empty() {
        return;
    };
    let n = array.len();
    let mut a = array[0].clone();
    let mut b: Rc<RefCell<Intersection<T>>>;
    for elem in array.iter().take(n).skip(1) {
        b = elem.clone();
        (*a).borrow_mut().n = Some(b.clone());
        (*b).borrow_mut().p = Some(a.clone());
        a = b;
    }
    b = array[0].clone();
    (*a).borrow_mut().n = Some(b.clone());
    (*b).borrow_mut().p = Some(a);
}
