import { geoPath, geoMercator, geoGraticule10, GeoPermissibleObjects } from "d3-geo";

export default async function mercator(world: GeoPermissibleObjects) {

  const canvas: HTMLCanvasElement = document.querySelector('#mercator-js');

  const context = canvas.getContext("2d");

  const width = canvas.width;
  const height = canvas.height;

  const projection = geoMercator()
    .scale(width / 1.3 / 3.14)
    .translate([width / 2, height / 2])
    .clipAngle(90)
    .precision(10);

  const path = geoPath(projection, context);

  context.strokeStyle = '#69b2a2';
  context.stroke();
  path(world);
  context.stroke();
  path(geoGraticule10());
  context.stroke();

}
