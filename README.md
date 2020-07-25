# rust_d3_geo


This is a port of the [d3-geo](https://github.com/d3/d3-geo) library into a RUST library crate/package. It is in a very early development phase. That is only one sub module 'rotaions' has gone through phase 1.

## Phase 1

Early draft port -  sub module by submodule. Sub module porting means the test have also been ported.
No API stability guarentees.

Next sub modules on the roadmap -

* Clip, GeoStream, Projection.

* Stereographic. [A raw projection.]

I am trying to get a program of mine to run faster, but I want this to eventually be a true library port. So feel free to add suggestions to my roadmap.

## Phase 2

API finialization. There maybe optimisation in the area of generics. So the API only gets locked down in phase 2.
 The code will be profiled and bottlenecks identified.


