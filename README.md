# WGPU texture sharing with DMA-BUF
## Requirements
- Linux OS
- EGL extensions:
  - EGL_MESA_image_dma_buf_export
  - EGL_EXT_image_dma_buf_import

## Goal
Trying to export an OpenGL texture to DMA BUF using the WGPU hal api.
Should be possible as this C example works: https://gitlab.com/blaztinn/dma-buf-texture-sharing/-/blob/master/main.c  

## Problem 
The `egl_export_dmabufimage_query_mesa` function returns status 0
