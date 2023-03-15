
import { geoPath, geoAlbers, geoGraticule10 } from "d3-geo";

let grid = document.getElementById("grid");
let width = 1200;
let height = 1200;
const projection = geoAlbers()
  .scale(width / 1.5 / 3.14)
  .translate([width / 2, height / 2]);


const path = geoPath(projection);
const d = path(geoGraticule10());

console.log(d);

grid.setAttribute('d', d);