# WGPU texture sharing via DMA-BUF to Glutin and Slint 
## Requirements
- Linux OS
- Wgpu and Slint configured using OpenGL backends
- EGL extensions:
  - EGL_MESA_image_dma_buf_export
  - EGL_EXT_image_dma_buf_import

## Problem
I was not able to use Wgpu content in a Slint image on Linux.

## Solution
Export an OpenGL texture to DMA-BUF using the WGPU hal api.  
Should be possible as this C example works: https://gitlab.com/blaztinn/dma-buf-texture-sharing/-/blob/master/main.c


## How It Works
- access the WGPU OpenGL texture via the hal api
- export it in a dma_buf_fd using eglExportDMABUFImageMESA
- I used two separate processes connected using Unix Datagram sockets
- read the texture back via glEGLImageTargetTexture2DOES
- use BorrowedOpenGLTextureBuilder to create the Slint Image or map it via a shader in Glutin

## How to Run
Cross process:
- Run `sender` using env `WGPU_BACKEND=gl` 
- Run one of:
  - `receiver_glutin`
  - `receiver_slint` using env `SLINT_BACKEND=GL`

Same process:

- Run `slint_wgpu_same_process` using both env variables as mentioned above

> [!NOTE]
> I am not a Rust developer nor a graphics programmer, tips are always welcome! 
