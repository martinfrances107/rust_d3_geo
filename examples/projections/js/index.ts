import { feature } from "topojson-client";
import { Topology } from "topojson-specification";

import albers from "./albers";
import albersUSA from "./albersUSA";
import azimuthalEqualArea from "./azimuthalEqualArea";
import azimuthalEquidistant from "./azimuthalEquidistant";
import conformal from "./conformal";
import conicEqualArea from "./conicEqualArea";
import equalEarth from "./equalEarth";
import equidistant from "./equidistant";
import equirectangular from "./equirectangular";
import gnomic from "./gnomic";
import mercator from "./mercator";
import mercatorTransverse from "./mercatorTransverse";
import orthographic from "./orthographic";
import stereographic from "./stereographic";

// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
import("../pkg")
  .then((m) => {
    console.log("wasm is imported");

    m.start();
    fetch("../world-atlas/world/50m.json")
      .then((response) => response.json())
      .then((_worldTopo) => {
        const worldTopo = _worldTopo as Topology;
        const world = feature(worldTopo, worldTopo.objects.countries);
        Promise.all([
          albers(world),
          albersUSA(world),
          azimuthalEqualArea(world),
          azimuthalEquidistant(world),
          conformal(world),
          conicEqualArea(world),
          equirectangular(world),
          equidistant(world),
          equalEarth(world),
          gnomic(world),
          mercator(world),
          mercatorTransverse(world),
          orthographic(world),
          stereographic(world),
        ]);
      });
  })
  .catch(console.error);
