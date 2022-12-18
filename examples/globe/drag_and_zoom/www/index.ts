import { ExportedPoint, Renderer } from '../pkg/rust_d3_geo_example_globe_drag_and_zoom'
import { drag } from 'd3-drag'
import { pointer, select } from 'd3-selection'
import { eulerAngles } from './mathsfunctions.js'

const perf = document.getElementById('perf')

if (perf != null) {
  perf.innerHTML = 'Render Time: ...Calculating'

  console.log('wasm is imported')
  Renderer.new('./world-atlas/world/50m.json')
    .then((renderer: Renderer) => {
      const canvasArray = document.getElementsByTagName('canvas')
      if (canvasArray.length !== 1) {
        return
      }

      const canvas = canvasArray[0]

      let o0 /// starting rotation.
      let gpos0 = Array(2).fill(0)
      let gpos1 = Array(2).fill(0)
      const d3Canvas = select('#c')

      function dragstarted (e: any) {
        // console.log('drag started')
        const canvasxy = pointer(e)

        const inverted = renderer.invert(new ExportedPoint(canvasxy[0], canvasxy[1]))
        gpos0 = [inverted.x, inverted.y]
        const sLat = document.getElementById('lat')
        const sLong = document.getElementById('long')

        if (sLat === null || sLong === null) {
          return
        }
        sLat.innerText = gpos0[0]
        sLong.innerText = gpos0[1]

        o0 = renderer.rotate()
        const s_rotation = document.getElementById('rotation')
        if (s_rotation == null) {
          return
        }
        s_rotation.innerText = o0[0] + ' , ' + o0[1] + ' , ' + o0[2]
        renderLoop()
      }

      function dragged (e: any) {
        // console.log('dragged')

        // canvas is needed here as a input, not in dragstarted 
        // no sure why...
        const canvasxy = pointer(e, canvas)

        const inverted = renderer.invert(new ExportedPoint(canvasxy[0], canvasxy[1]))
        gpos1 = [inverted.x, inverted.y]

        o0 = renderer.rotate()

        const o1 = eulerAngles(gpos0, gpos1, o0)
        if (o1 === undefined) {
          console.log('oops failed.')
          return
        } else {
          if (o1.length !== 3) {
            console.log('not 3')
            return
          }
        }
        // console.log("o1", o1);
        renderer.rotate_set(o1)
        renderLoop()
      }

      function dragended (e: any) {
        // console.log('drag end')
      }

      const drag2 = drag()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended)

      d3Canvas.call(drag2)

      const context = canvas.getContext('2d')
      if (context == null) {
        return
      }

      const renderLoop = () => {
        console.log('render loop')
        context.clearRect(0, 0, 1800, 1200)

        renderer.render(false)
        renderer.render_point(gpos0[0], gpos0[1])
        renderer.render_point(gpos1[0], gpos1[1])
      }

      renderLoop()
    })
}
