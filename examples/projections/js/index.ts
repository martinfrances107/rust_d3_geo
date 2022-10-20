
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
import conicEqualArea from "./conicEqualArea";
import equirectangular from './equirectangular';
import gnomic from "./gnomic";
import orthographic from './orthographic';
import mercator from './mercator';
import mercatorTransverse from './mercatorTransverse';
import stereographic from './stereographic';

fetch("../world-atlas/world/50m.json").then(response => response.json())
  .then(_worldTopo => {
    const worldTopo = _worldTopo as Topology;
    const world = feature(worldTopo, worldTopo.objects.countries);
    Promise.all(
      [
        azimuthalEqualArea(world),
        azimuthalEquidistant(world),
        conicEqualArea(world),
        equirectangular(world),
        orthographic(world),
        gnomic(world),
        mercator(world),
        mercatorTransverse(world),
        stereographic(world),
      ]
    )
  });
