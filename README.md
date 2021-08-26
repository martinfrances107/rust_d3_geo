# rust_d3_geo

This is a port of the [d3-geo](https://github.com/d3/d3-geo) library into a RUST library crate/package. It is in a very early development phase. That is only sub modules 'rotaions' and 'polygon_contains' has gone through phase 1.

## Phase 1

Early draft port -  sub module by submodule. Sub module porting means the test have also been ported.
No API stability guarentees.

Progress so far.

A list of all projections can be found in invert-test.rs. Out of the 15 distinct projections listed only 6 have been ported so far.
(AziumuthalEqualArea, Equirectangular, Gnomic, Mercator,  Orthographic, Stereographic )

## TODO list

Currently we have 47% test coverage ( as reported by cargo tarpaulin -o Html )
* The current target is to increase the code coverage in resample.rs which has some known bugs.
  clipcircle_test.ts and fit_test.rs exercise code in resample .. So I am focusing on those tests.
* Next,  sections of projectionMutator.rs will have to be re-architected for the mercator projection to function like the javascript version.

I am trying to get a program of mine to run faster, but I want this to eventually be a true library port. So feel free to add suggestions to my todo list.

## Phase 2

API finialization. There maybe optimisation in the area of generics. So the API only gets locked down in phase 2.
 The code will be profiled and bottlenecks identified.

Modules, passing test ready for phase 2 evaluation :-

* rotation
* projection [ stream() does not yet cache the return value ].
* The code in stream_dst.rs may need to be simplified.
