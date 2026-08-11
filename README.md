# Nalvik

A software rasterizer written in rust.

## Features

- [x] Fully generic vertex data
- [x] Fully programmable vertex and fragment shaders
- [x] Supports both offline and online rendering
- [x] Depth buffering
- [x] Configurable back-face Culling
- [x] Perspective correct interpolation
- [x] Multithreading
  - [x] Primitive Assembly
  - [x] Rasterization
- [x] Texture loading
- [ ] Index buffering
- [ ] Texture sampling
  - [x] Nearest filtering
  - [ ] Bilinear filtering
  - [ ] Mipmapping and trilinear filtering
- [ ] SIMD

## Examples

Examples use [winit](https://docs.rs/winit/0.30.13/winit/) and [pixels](https://docs.rs/pixels/0.17.2/pixels/)

Click inside the window to enable focus. Use wasd to move horizontally. E to move up and Q to move down. Move the cursor to control the viewing direction.

- [Height Map Terrain](https://github.com/mcthouacbb/Nalvik/tree/main/examples/height_map_terrain)
- [Obj Model Loader](https://github.com/mcthouacbb/Nalvik/tree/main/examples/model_loader)
- [Portal Renderer](https://github.com/mcthouacbb/Nalvik/tree/main/examples/portal_renderer)

The name is based on reversing "vulkan" -> "nakluv"
