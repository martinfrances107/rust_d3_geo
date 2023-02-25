
import { geoPath, geoEqualEarth, geoGraticule10, GeoPermissibleObjects } from "d3-geo";

export default async function equalEarth (world: GeoPermissibleObjects) {

  const canvas: HTMLCanvasElement = document.querySelector('#equal-earth-js');

  const context = canvas.getContext("2d");

  const width = canvas.width;
  const height = canvas.height;

  const projection = geoEqualEarth()
    .scale(width / 1.5 / Math.PI)
    .rotate([0, 0])
    .center([0, 0])
    .translate([width / 2, height / 2])

  const path = geoPath(projection, context);

  context.strokeStyle = '#69b2a2';
  path(world);
  context.stroke();
  path(geoGraticule10());
  context.stroke();

}
