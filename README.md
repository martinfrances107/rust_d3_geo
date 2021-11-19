# Rust D3 Geo
Rust 2021 Edition.

This is a port of the [d3-geo](https://github.com/d3/d3-geo) library into a RUST library crate/package. It is in a very early development phase.

We are currently in phase two (Performance testing.)

* Phase One was code correctness.[Complete November 2021.]
* Phase Two is about performance.
* Phase Three sweeping up infrequently used code and making the library complete.

Currently we have 83% code coverage ( as reported by cargo tarpaulin )

## Phase One ( Code Corectness ) completed.

The bulk of the code has been ported over and has been verified by porting over the test code.

## Phase Two

A) Initial code profing shows that much of the time is spent in mem-copy.

* In places I may be passing large data object into functions instead of pass by reference.
* Hashmaps - appear slow.
  - The observed  constant resizing delays can be reduced if the initial capacity is set.
  - Maybe the hashing algorithm can be improved.

B) API finialization. There maybe optimisation in the area of generics. So the API only gets locked down in phase 2.
 The code will be profiled and bottlenecks identified.

## Phase Three

D3 geo uses TopoJson is in testing. We need to integrate with  [topojson](https://docs.rs/topojson/0.5.0/topojson/) to complete code coverage.


A list of all ported projections can be found in invert-test.rs. Out of the 15 distinct projections listed only 6 have been ported so far.

* AziumuthalEqualArea
* Equirectangular
* Gnomic
* Mercator
* Orthographic
* Stereographic

I am trying to get a program of mine to run faster, but I want this to eventually be a true library port. So feel free to add suggestions to my todo list.

## Other To-do's

### Document API changes such as
  * src/projection/clip_angle_reset()
  * src/projection/clip_extent_clear()

### To be implemented
  path/measure and measure-test.js
  Identity.js and Identity-test.js

### Missing test
  area_test.rs: The stripes() function needs to be added so tests such as 'area: Polygon - stripes -45°, 45°' can be implmented.

