
import { geoPath, geoEquirectangular, geoGraticule10, GeoPermissibleObjects } from "d3-geo";

export default async function equirectangular(world: GeoPermissibleObjects) {

  const canvas: HTMLCanvasElement = document.querySelector('#equirectangular-js');

  const context = canvas.getContext("2d");

  const width = canvas.width;
  const height = canvas.height;

  var projection = geoEquirectangular()
    .scale(width / 1.5 / Math.PI)
    .rotate([0, 0])
    .center([0, 0])
    .translate([width / 2, height / 2])

  var path = geoPath(projection, context);

  context.strokeStyle = '#69b2a2';
  path(world);
  context.stroke();
  path(geoGraticule10());
  context.stroke();

}
