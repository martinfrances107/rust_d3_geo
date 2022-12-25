import { ExportedPoint, Renderer } from '../pkg/rust_d3_geo_example_globe_drag_and_zoom'
import { drag } from 'd3-drag'
import { pointer, select } from 'd3-selection'
import { eulerAngles } from './mathsfunctions.js'

class Clamp {
  #min: number
  #max: number
  constructor(min: number, max: number) {
    this.#min = min
    this.#max = max
  }

  apply (x: number): number {
    return Math.min(Math.max(this.#min, x), this.#max)
  }
}

const clamp = new Clamp(400, 900)

let scale: number
let isSolid: boolean

console.log('wasm is imported')
Renderer.new('./world-atlas/world/50m.json')
  .then((renderer: Renderer) => {
    const canvasArray = document.getElementsByTagName('canvas')
    if (canvasArray.length !== 1) {
      return
    }

    const canvas = canvasArray[0]

    scale = renderer.scale()
    const zoom = (event: WheelEvent): void => {
      event.preventDefault()

      scale += event.deltaY * -0.5

      // Restrict scale.
      scale = clamp.apply(scale)

      // Apply scale transform.
      renderer.scale_set(scale)
      renderLoop()
    }

    const isSolidElem = document.getElementById('is_solid')
    let isSolidInput: HTMLInputElement
    if (isSolidElem === null) {
      return
    } else {
      isSolidInput = isSolidElem as HTMLInputElement
    }

    isSolidElem.addEventListener('click', (e) => {
      isSolid = isSolidInput.checked
      renderLoop()
    })

    canvas.onwheel = zoom

    let o0: number[] /// starting rotation.
    let gpos0 = new ExportedPoint(0, 0)
    let gpos1 = new ExportedPoint(0, 0)
    const d3Canvas = select('#c')

    function dragstarted (e: any): void {
      const canvasxy = pointer(e)

      const gpos0 = renderer.invert(new ExportedPoint(canvasxy[0], canvasxy[1]))

      o0 = renderer.rotate()
      const sRotation = document.getElementById('rotation')
      if (sRotation == null) {
        return
      }
      sRotation.innerText = `${o0[0]} , ${o0[1]} , ${o0[2]} `
      renderLoop()
    }

    function dragged (e: any): void {
      // canvas is needed here as a input, not in dragstarted
      // no sure why...
      const canvasxy = pointer(e, canvas)

      const gpos1 = renderer.invert(new ExportedPoint(canvasxy[0], canvasxy[1]))

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

      renderer.rotate_set(o1)
      renderLoop()
    }

    function dragended (e: any): void {
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

    const renderLoop = (): void => {
      context.clearRect(0, 0, 1800, 1200)

      renderer.render(isSolid)
      renderer.render_point(gpos0.x, gpos0.y)
      renderer.render_point(gpos1.x, gpos1.y)
    }

    renderLoop()
  }).catch((e) => { console.log('Did not receive Renderer', e) })
