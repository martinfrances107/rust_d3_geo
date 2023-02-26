
// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('../pkg');

rust
  .then(m => {
    console.log("wasm is imported");
    m.start()
  })
  .catch(console.error);

import { feature } from "topojson-client";
import { Topology } from "topojson-specification";

import azimuthalEqualArea from './azimuthalEqualArea';
import azimuthalEquidistant from './azimuthalEquidistant';
import conformal from './conformal';
import conicEqualArea from "./conicEqualArea";
import equirectangular from './equirectangular';
import equalEarth from './equalEarth';
import gnomic from "./gnomic";
import orthographic from './orthographic';
import mercator from './mercator';
import mercatorTransverse from './mercatorTransverse';
import stereographic from './stereographic';
import albers from "./albers";

fetch("../world-atlas/world/50m.json").then(response => response.json())
  .then(_worldTopo => {
    const worldTopo = _worldTopo as Topology;
    const world = feature(worldTopo, worldTopo.objects.countries);
    Promise.all(
      [
        albers(world),
        azimuthalEqualArea(world),
        azimuthalEquidistant(world),
        conformal(world),
        conicEqualArea(world),
        equirectangular(world),
        equalEarth(world),
        orthographic(world),
        gnomic(world),
        mercator(world),
        mercatorTransverse(world),
        stereographic(world),
      ]
    )
  });
