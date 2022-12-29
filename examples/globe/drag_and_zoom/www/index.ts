import { ExportedPoint, Renderer } from '../pkg/rust_d3_geo_example_globe_drag_and_zoom'
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
let isMouseDown: boolean = false;

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
      isMouseDown = true;
      const canvasxy = pointer(e)
      const gposLast = gpos0;
      gpos0 = renderer.invert(new ExportedPoint(canvasxy[0], canvasxy[1]))
      renderLoop()
    }

    function dragged (e: any): void {
      if (isMouseDown === true) {
        const canvasxy = pointer(e)
        gpos1 = renderer.invert(new ExportedPoint(canvasxy[0], canvasxy[1]))
        console.log("updated gpos1", gpos1.x, gpos1.y)
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
    }

    function dragended (e: any): void {
      isMouseDown = false;
    }

    const context = canvas.getContext('2d')
    if (context == null) {
      return
    }
    select(context.canvas).on('mousemove', dragged)
      .on('mousedown', dragstarted)
      .on('mouseup', dragended)

    const renderLoop = (): void => {
      context.clearRect(0, 0, 1800, 1200)

      renderer.render(isSolid)
    }

    renderLoop()
  }).catch((e) => { console.log('Did not receive Renderer', e) })
