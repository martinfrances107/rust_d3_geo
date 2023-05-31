# The Intersection Problem

There is an aspect of the design that needs review. It related to the best way to implement a doubly-linked list which has cross links between nodes.

The original paper from which the clipping algorithm was developed is
[here](https://www.inf.usi.ch/hormann/papers/Greiner.1998.ECO.pdf )

Figure 10, shows two doubly-linked listed "clip" and "subject". Which can additional be charactersised as having "intersection" cross links joining the two linked lists as need

From the javasript version

[rejoin.js](https://github.com/d3/d3-geo/blob/main/src/clip/rejoin.js) Intersection has next and previous pointer, (n and p ), which is the pattern of a doubly-linked list

```js
function Intersection(point, points, other, entry) {
  this.x = point;
  this.z = points;
  this.o = other; // another intersection
  this.e = entry; // is an entry?
  this.v = false; // visited
  this.n = this.p = null; // next & previous
}
```

This is what the current naive rust implementation looks like.

```rust
pub struct Intersection<'a, T>
where
    T: CoordFloat,
{
    pub x: LineElem<T>,
    pub z: Option<&'a Vec<LineElem<T>>>,
    /// Another intersection.
    pub o: Option<Rc<RefCell<Intersection<'a, T>>>>,
    /// is any entry?
    pub e: bool,
    /// visited.
    pub v: bool,

    /// Next.
    pub n: Option<Rc<RefCell<Intersection<'a, T>>>>,
    /// Previous.
    pub p: Option<Rc<RefCell<Intersection<'a, T>>>>,
}
```

Because of rusts unique ownership model, doubly-linked list must be used with care. The general recommendation is to not use them unless one has a clear set of reasoning as to why nothing else will work.

In Rust doubly-linked list are found here [std::collections::LinkedList](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)

* support is experimental
* In std, functionality is limited.

 Support for doubly-linked list with insert, remove and split functionaility
 can be found outside std, here [intrusive_collections::linked_list::CursorMut](intrusive_collections::linked_list::CursorMut)
