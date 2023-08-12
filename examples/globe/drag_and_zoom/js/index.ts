import { pointer, select } from 'd3-selection';
import { eulerAngles } from '../mathsfunctions';
import { SelectedPattern } from '../pkg';


import('../pkg')
  .then(pkg => {
    console.log('wasm is imported')
    var scale: number = 1;
    var isSolid: boolean = false;
    var isMouseDown: boolean = false;
    const selectElement = document.querySelector<HTMLSelectElement>('#select_pattern');
    if (selectElement === null) {
      console.log("failed to find #show_rings");
      return
    } else {

      let initial_selected_pattern;
      switch (selectElement.options[selectElement.selectedIndex].value) {
        case "bar":
          initial_selected_pattern = pkg.SelectedPattern.Bar;
          break
        case "globe":
          initial_selected_pattern = pkg.SelectedPattern.Globe;
          break;
        default:
          console.log("settings rings");
          initial_selected_pattern = pkg.SelectedPattern.Rings;
          break;
      }


      pkg.Renderer.new(initial_selected_pattern)
        .then((renderer) => {
          let pattern_promise = pattern_set();

          const canvasArray = document.getElementsByTagName('canvas')
          if (canvasArray.length !== 1) {
            return
          }

          const canvas = canvasArray[0]

          const context = canvas.getContext('2d')
          if (context == null) {
            return
          }

          let selector = select(context.canvas);

          scale = renderer.scale()
          const zoom = (event: WheelEvent): void => {

            scale += event.deltaY * -0.5

            // Restrict scale.
            scale = Math.min(Math.max(400, scale), 900)

            // Apply scale transform.
            renderer.scale_set(scale)
            renderLoop()
          }

          const isSolidElem = document.querySelector<HTMLInputElement>('#is_solid')
          if (isSolidElem === null) {
            console.log("failed to find #is_solid");
            return
          }

          isSolidElem.addEventListener('click', (_e) => {
            isSolid = isSolidElem.checked
            renderLoop()
          })

          selectElement.addEventListener('change', (_event) => {
            pattern_set().then(() => {
              renderLoop()
            }).catch((e) => {
              console.log("white updating the pattern selector");
              console.log(e);
            });
          });

          canvas.onwheel = zoom

          let o0: number[] /// Starting rotation.
          let gpos0 = new pkg.ExportedPoint(0, 0)
          let gpos1 = new pkg.ExportedPoint(0, 0)

          function dragstarted (e: any): void {
            isMouseDown = true

            selector.on('mousemove', dragged)
              .on('mouseup', dragended);

            const canvasxy = pointer(e)
            gpos0 = renderer.invert(new pkg.ExportedPoint(canvasxy[0], canvasxy[1]))
          }

          // Set the pattern based on the input selection
          //
          // async: underlying RUST file load call is async
          // errors if the file is not found on disk.
          function pattern_set (): Promise<void> {

            const value = selectElement?.options[selectElement.selectedIndex].value;
            console.log(value);

            let promise;
            switch (value) {
              case "bar":
                console.log("settings bar");
                promise = renderer.pattern_change(pkg.SelectedPattern.Bar);
                break
              case "globe":
                console.log("settings globe");
                promise = renderer.pattern_change(pkg.SelectedPattern.Globe);
                break;
              default:
                console.log("settings rings");
                promise = renderer.pattern_change(pkg.SelectedPattern.Rings);
                break;
            }
            return promise;
          }

          function dragged (e: any): void {
            if (isMouseDown) {
              const canvasxy = pointer(e)
              gpos1 = renderer.invert(new pkg.ExportedPoint(canvasxy[0], canvasxy[1]))
              o0 = renderer.rotate()

              const o1 = eulerAngles(gpos0, gpos1, o0)
              if (o1 === undefined) {
                console.log('oops failed.')
                console.log(gpos0);
                console.log(gpos1);
                console.log(o0);
                return;
              } else {
                if (o1.length !== 3) {
                  console.log('not 3')
                  return;
                }
              }

              renderer.rotate_set(o1)
              renderLoop();
            }
          }

          function dragended (e: any): void {

            isMouseDown = false

            canvas.removeEventListener("mousemove", dragged);
            canvas.removeEventListener("mousedown", dragended);
          }

          select(context.canvas)
            .on('mousedown', dragstarted)

          const renderLoop = (): void => {
            context.clearRect(0, 0, 1800, 1200)

            renderer.render(isSolid)
          }

          pattern_promise.then(() => {
            renderLoop()
          }).catch((e) => {
            console.error("Initialization: Could not load pattern");
            console.error(e);
          });

        }).catch((e) => { console.error('Did not receive Renderer', e) })
    }
  })