import { pointer, select } from 'd3-selection'
import { eulerAngles } from '../mathsfunctions'

import('../pkg')
  .then(pkg => {
    console.log('wasm is imported')
    var scale: number = 1
    var isSolid: boolean = false
    var isMouseDown: boolean = false
    pkg.Renderer.new('./world-atlas/world/50m.json')
      .then((renderer) => {
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
          scale = Math.min(Math.max(400, scale), 900)

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

        let o0: number[] /// Starting rotation.
        let gpos0 = new pkg.ExportedPoint(0, 0)
        let gpos1 = new pkg.ExportedPoint(0, 0)

        function dragstarted (e: any): void {
          isMouseDown = true
          const canvasxy = pointer(e)
          gpos0 = renderer.invert(new pkg.ExportedPoint(canvasxy[0], canvasxy[1]))
        }

        function dragged (e: any): void {
          if (isMouseDown) {
            const canvasxy = pointer(e)
            gpos1 = renderer.invert(new pkg.ExportedPoint(canvasxy[0], canvasxy[1]))
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
          isMouseDown = false
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

  })