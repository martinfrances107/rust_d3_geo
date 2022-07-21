# Rust D3 Geo
Rust 2021 Edition.

This is a port  [d3-geo](https://github.com/d3/d3-geo) into RUST.

## Status
The majority of the libray has been ported along with the associated tests. The aim is to eventaully release this as a rust crate.
but at the moment is this alpha grade software.

Here is a list of thing todo before the crate is published.

* The recenter state-based API refacor is almost complete.
  Once fit_size_resampling() is reinstated the code-coverage metric will jump 10% back to the previous value of approx 82%
* The clipping algorithm in clip/rejoin/mod.rs needs to be refactored.
see  [The intersection Problem.](#the-intersection-problem--request-for-comments)
Test coverage in that area is high so the algortihms is working but the data structures make extensive use of Vec<Options<Rc<RefCell<_Intersection_>>>> which is not performant.

* The API is not stabalised. If perfomance issues arise then the API will change. Additionaly I plan a final review to remove anything uneeded before making changes become complicated.

## Examples

In addition to the port, some examples are provided to help developers convert their existing javascript to rust.

| Name | Description|
--- | ---|
| examples/globe/canvas | Shows how to load/parse/display  a complex topojson file to a CANVAS element.|
| examples/globe/svg  | Shows how to load/parse/display the globe as indivdual SVG PATH elements. Useful when the semantic meaing of the data needs to be preserved.|
| examples/projections | Shows a side by side comparison of the all the projections rendered by both javascript and rust.|
| examples/graticule | Show various ways of rendering latitide and longitude lines.|
| examples/ring | Renders a complex multipolygon.|


## The Intersection Problem ( Request for comments )

There is an aspect of the design that needs review. It related to the best way to implement a doubly-linked list which has cross links between nodes.


The original paper from which the clipping algorithm was developed is
[here](https://www.inf.usi.ch/hormann/papers/Greiner.1998.ECO.pdf )

Figure 10, shows two doubly-linked listed "clip" and "subject". Which can additional be charactersised as having "intersection" cross links joining the two linked lists as need

From the javasript version

[rejoin.js](https://github.com/d3/d3-geo/blob/main/src/clip/rejoin.js) Intersection has next and previous pointer, (n and p ),  which is the pattern of a doubly-linked list

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
    #[derivative(Debug = "ignore")]
    /// Another intersection.
    pub o: Option<Rc<RefCell<Intersection<'a, T>>>>,
    /// is any entry?
    pub e: bool,
    /// visited.
    pub v: bool,
    #[derivative(Debug = "ignore")]
    /// Next.
    pub n: Option<Rc<RefCell<Intersection<'a, T>>>>,
    #[derivative(Debug = "ignore")]
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

## Benchmarking

In this project, we have two benchmarks, based on the ring and graticule examples ( see above. )

Also [rust_d3_geo_voronoi](https://github.com/martinfrances107/rust_d3_geo_voronoi)
 uses this library, and thank project contains a benchmark which contains an exact port of a benchmark in [d3-geo-voronoi ](https://github.com/Fil/d3-geo-voronoi).
 Based on that benchmark rust is 31% faster, or permits a 37% increase in throughput.

## Multi thread support

On my todo list.
* [rayon](https://docs.rs/rayon/latest/rayon/index.html) is rust's crate for multithread support.
I have made extensive use of iterators when porting the code and rayon support the easy conversion of single threaded iterators to multithread iterators.

* The Hashmaps - appear slow.
  Maybe I can get performace improvements by replacing them with B-tree collections?

## Unimplemented sections of the library.

Support for a custom projection is not yet supported.
For an example of this see the test labelled "projection.fitExtent(…) custom projection"

I am trying to get a program of mine to run faster, but I want this to eventually be a true library port. So feel free to add suggestions to my todo list.

Here is a list of the currently supported projections.
* AziumuthalEqualArea
* AzimuthalEquiDistant
* Equirectangular
* Gnomic
* Mercator
* Orthographic
* Stereographic

A complete list of all ported projections can be found in invert-test.rs. Out of the 15 distinct projections listed only 7 have been ported so far.

## Other To-do's

### Document API changes such as
  * src/projection/clip_angle_reset()
  * src/projection/clip_extent_clear()

### To be implemented
  * path/measure and measure-test.js
  * Identity.js and Identity-test.js

Finally

todo.md contains a more detailed todo list.
