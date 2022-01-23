# Rust D3 Geo
Rust 2021 Edition.

This is a port of the [d3-geo](https://github.com/d3/d3-geo) library into RUST. The majority of the libray has been ported along with the associated tests.

In addtion some demonstration applications have been provided in the example directory to help developers convert their existing javascript to rust.

| Name | Description|
--- | ---|
| examples/globe/canvas | Shows how to load/parse/display  a complex topojson file to a CANVAS element.|
| examples/globe/svg  | Shows how to load/parse/display the globe as indivdual SVG PATH elements. Useful when the semantic meaing of the data needs to be preserved.|
| examples/projections | Shows a side by side comparison of the all the projections rendered by both javascript and rust.|
| examples/graticule | Show various ways of rendering latitide and longitude lines.|
| examples/ring | Renders a complex multipolygon.|

The current focus is now on benchmarking and making the library multithreaded.

The API is not stabalised. If perfomance issues arise then the API will change.

Finally, the infrequently used projections and other minor items need be implemented to make the library complete.

Currently we have 84% code coverage ( as reported by cargo tarpaulin )


## Notes on performance

This code performs only slighty better than the JS version. So I am looking into making this library multi threaded.

* I am about to implement benchmarking.

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

todo.md contains a more detailed todo list.
