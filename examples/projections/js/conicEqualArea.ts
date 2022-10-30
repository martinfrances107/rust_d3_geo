import { geoPath, geoConicEqualArea, geoGraticule10, GeoPermissibleObjects } from "d3-geo";

export default async function conicEqualArea(world: GeoPermissibleObjects) {

  const canvas: HTMLCanvasElement = document.querySelector('#conic-equal-area-js');

  const context = canvas.getContext("2d");

  const width = canvas.width;
  const height = canvas.height;

  const projection = geoConicEqualArea()
    .translate([width / 2, height / 2])

  const path = geoPath(projection, context);

  context.strokeStyle = '#69b2a2';
  path(world);
  context.stroke();
  path(geoGraticule10());
  context.stroke();

}
