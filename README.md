# Rust D3 Geo

This is a port of the [d3-geo](https://github.com/d3/d3-geo) library into a RUST library crate/package. It is in a very early development phase.

* Phase One is about code correctness.
* Phase Two is about performance
* Phase Three sweeping up infrequently used code and making the library complete.

Currently we have 62% code coverage ( as reported by cargo tarpaulin -o Html )

Early indications show that the performace is only three times better than the original javascript [ I think a eight times speedup is a good long term target].

## Phase One

Early draft port -  sub module by submodule. Sub module porting means the test have also been ported.
No API stability guarentees.

### Phase One todos
Code coverage in the clip mod stands at only 25%.
so I am working on porting over test into fit_test.rs and mercator_test.rs

## Phase Two

A) Initial code profing shows that much of the time is spent in mem-copy.

* In places I may be passing large data object into functions instead of pass by reference.
* Hashmaps. They appear slow.
  - The observed  constant resizing delays can be reduced if the initial capacity is set.
  - Maybe the hashing algorithm can be improved.


B) API finialization. There maybe optimisation in the area of generics. So the API only gets locked down in phase 2.
 The code will be profiled and bottlenecks identified.

Modules, passing test ready for phase 2 evaluation :-

* rotation
* projection [ stream() does not yet cache the return value ].
* The code in stream_dst.rs may need to be simplified.

## Phase Three

D3_geo uses TopoJson is in testing. We need to integrate with  [topojson](https://docs.rs/topojson/0.5.0/topojson/) to complete code coverage.


A list of all projections can be found in invert-test.rs. Out of the 15 distinct projections listed only 6 have been ported so far.

* AziumuthalEqualArea
* Equirectangular
* Gnomic
* Mercator
* Orthographic
* Stereographic

I am trying to get a program of mine to run faster, but I want this to eventually be a true library port. So feel free to add suggestions to my todo list.
