# [WGPU](https://docs.rs/wgpu/latest/wgpu/) Primer

A Glossary of term, Plus a outline of common code patterns
Code patterns

## Glossary
* [Queue](https://docs.rs/wgpu/latest/wgpu/struct.Queue.html#) - Command Queue on the device.
* [CommandEncoder](https://docs.rs/wgpu/latest/wgpu/struct.CommandEncoder.html#) - accepts command in RUST format.
  * configures how data will flow between [Buffers](https://docs.rs/wgpu/latest/wgpu/struct.Buffer.html#)  and [Textures](https://docs.rs/wgpu/latest/wgpu/struct.Texture.html#).
* [RenderPass](https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html#)    * "In-progress recording of a render pass: a list of render commands in a CommandEncoder."
* [Instance](https://docs.rs/wgpu/latest/wgpu/struct.Instance.html#)
  * root level wrapper "Context for all other wgpu objects."

* [Instancing](https://sotrh.github.io/learn-wgpu/beginner/tutorial7-instancing/)
    * "Instancing allows us to draw the same object multiple times with different properties (position, orientation, size, color, etc.)"
* [Surface](https://docs.rs/wgpu/latest/wgpu/struct.Surface.html#)
  * A Surface represents a platform-specific surface (e.g. a window) onto which rendered images may be presented.

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

#### Data structure?

#### How to program the commnd encoder?