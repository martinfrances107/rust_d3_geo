# [WGPU](https://docs.rs/wgpu/latest/wgpu/) Primer

A Glossary of term, Plus a outline of common code patterns
Code patterns

## Glossary

* [Queue](https://docs.rs/wgpu/latest/wgpu/struct.Queue.html#) - Command Queue on the device.
* [CommandEncoder](https://docs.rs/wgpu/latest/wgpu/struct.CommandEncoder.html#) - accepts command in RUST format.
  * configures how data will flow between [Buffers](https://docs.rs/wgpu/latest/wgpu/struct.Buffer.html#)  and [Textures](https://docs.rs/wgpu/latest/wgpu/struct.Texture.html#).

* EBO Element array Buffer?

* [RenderPass](https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html#)    * "In-progress recording of a render pass: a list of render commands in a CommandEncoder."
* [Instance](https://docs.rs/wgpu/latest/wgpu/struct.Instance.html#)
  * root level wrapper "Context for all other wgpu objects."

* [Instancing](https://sotrh.github.io/learn-wgpu/beginner/tutorial7-instancing/)

    * "Instancing allows us to draw the same object multiple times with different properties (position,
    orientation, size, color, etc.)"

* [Primitive Restart](https://www.supergoodcode.com/restart/)
>     "The last remaining feature for GL 3.1 was primitive restart, which allows an indexed draw command to end the current primitive when a specified index is processed, beginning a new one of the same type with the next index. Other than the minor changes of enabling the driver capability, there were two main issues with translating this functionality to Vulkan"

* [Surface](https://docs.rs/wgpu/latest/wgpu/struct.Surface.html#)
  * A Surface represents a platform-specific surface (e.g. a window) onto which rendered images may be presented.

* Uniforms - Same for all runs

* attribue - unique per vertex


* VAO VBO
  * Good diagrams here  [What are VBO and VAO?](https://www.letsdevelopgames.com/2022/02/what-are-vbo-and-vao.html)

* Clip Space - Has  four dimensions: (x, y, z, w)
  * -1<x<1
  * -1<y<1
  * 0<z<1

* Texture Coordinates
  * 0 ≤ u ≤ 1.0
  * 0 ≤ v ≤ 1.0
  * 0 ≤ w ≤ 1.0

## Code Patterns

### Rendering the **Boids** example

 [fn render](https://github.com/gfx-rs/wgpu/blob/trunk/examples/src/boids/mod.rs#L269) (queue:  [Queue](https://docs.rs/wgpu/latest/wgpu/struct.Queue.html#) )

* Construct a new new encoder.
    * encoder: PUSH a new group.
    * encoder: Construct a render_pass.
        * render_pass: Setup the pipeline.
        * render_pass: Set a vertex buffer.
        * render_pass: Make multiple Draw() calls here.
    * encoder: POP - the group is now **closed** for additions.
* encode: Submit the encoder to the queue.

### How to store a series of poly-lines.
  The phase to google is "primitve restart fixed index"
  In stip mode
  "The processing of a object is Reset/Restarts. Meaning on receipt of a 0xFFFF_FFFF to end of an object and a next index is the start of a new object of the same type"
  [drawing polyline](https://www.youtube.com/watch?v=hUMXGxfG9ow&list=PL980gcR1LE3L8RoIMSNBFfw4dFfS3rrsk&index=3)
  is
  [search  here for primitive restart mode](https://wgpu.rs/doc/wgpu/struct.PrimitiveState.html?search=restart#structfield.polygon_mode)
#### Data structure?

#### How to program the commnd encoder?