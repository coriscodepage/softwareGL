use std::array;

use crate::enums::{DrawBufferFBO, DrawBufferSys, GL_MAX_COLOR_ATTACHMENTS};

// Not actually a u32. 32 single bit flags.
pub type GlBitfield = u32;
// C ABI does not have a bool. Any value != 0 counted as true
pub type GlBool = u8;
// A signed integer but clamped to [0; i32::MAX]
pub type GlSizei = i32;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ColorValue {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl ColorValue {
    pub(crate) fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

pub(crate) struct ColorBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<ColorValue>,
}

impl ColorBuffer {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![ColorValue::default(); width * height],
        }
    }
}

pub(crate) struct DepthBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<f32>,
}

impl DepthBuffer {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0f32; width * height],
        }
    }
}

pub(crate) struct StencilBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

pub(crate) struct DepthStencilBuffer {
    pub width: usize,
    pub height: usize,
    pub depth: Vec<f32>,
    pub stencil: Vec<u8>,
}

pub(crate) struct FBO {
    pub width: usize,
    pub height: usize,
    pub color_attachments: [ColorBuffer; GL_MAX_COLOR_ATTACHMENTS],
    pub depth_attachment: Option<DepthBuffer>,
    pub stencil_attachment: Option<StencilBuffer>,
    pub depth_stencil_attachment: Option<DepthStencilBuffer>,
    pub draw_buffers: [Option<DrawBufferFBO>; GL_MAX_COLOR_ATTACHMENTS],
}

impl FBO {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color_attachments: array::from_fn(|_| ColorBuffer::new(width, height)),
            depth_attachment: None,
            stencil_attachment: None,
            depth_stencil_attachment: None,
            draw_buffers: [None; GL_MAX_COLOR_ATTACHMENTS],
        }
    }
}

pub(crate) struct DefaultFramebuffer {
    pub width: usize,
    pub height: usize,
    pub color_buffer_front: ColorBuffer,
    pub color_buffer_back: ColorBuffer,
    pub depth_buffer: Option<DepthBuffer>,
    pub stencil_buffer: Option<StencilBuffer>,
    pub draw_buffer: DrawBufferSys,
}
impl DefaultFramebuffer {
    pub(crate) fn init(width: usize, height: usize, double_buffered: GlBool) -> Self {
        Self {
            width,
            height,
            color_buffer_front: ColorBuffer::new(width, height),
            color_buffer_back: ColorBuffer::new(width, height),
            depth_buffer: Some(DepthBuffer::new(width, height)),
            stencil_buffer: None,
            draw_buffer: if double_buffered != 0 {
                DrawBufferSys::Back
            } else {
                DrawBufferSys::Front
            },
        }
    }
    pub fn as_slice_u8(&self, buffer: DrawBufferSys) -> Vec<u8> {
        let capacity = self.height * self.width * 4;
        let mut ret = Vec::<u8>::with_capacity(capacity);
        match buffer {
            crate::enums::DrawBufferSys::Front | crate::enums::DrawBufferSys::FrontLeft => unsafe {
                ret.set_len(capacity);
                let ptr = ret.as_mut_ptr();

                for (i, pixel) in self.color_buffer_front.pixels.iter().enumerate() {
                    let base = i * 4;
                    *ptr.add(base) = (pixel.alpha * 255.0) as u8;
                    *ptr.add(base + 1) = (pixel.blue * 255.0) as u8;
                    *ptr.add(base + 2) = (pixel.green * 255.0) as u8;
                    *ptr.add(base + 3) = (pixel.red * 255.0) as u8;
                }
            },
            crate::enums::DrawBufferSys::Back | crate::enums::DrawBufferSys::BackLeft => unsafe {
                ret.set_len(capacity);
                let ptr = ret.as_mut_ptr();

                for (i, pixel) in self.color_buffer_back.pixels.iter().enumerate() {
                    let base = i * 4;
                    *ptr.add(base) = (pixel.alpha * 255.0) as u8;
                    *ptr.add(base + 1) = (pixel.blue * 255.0) as u8;
                    *ptr.add(base + 2) = (pixel.green * 255.0) as u8;
                    *ptr.add(base + 3) = (pixel.red * 255.0) as u8;
                }
            },
            _ => {}
        }
        ret
    }
}

trait Enabelable {
    fn enable(&mut self);
    fn disable(&mut self);
    fn get_state(&self) -> bool;
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Scissor {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    enabled: bool,
}

impl Scissor {
    pub(crate) fn new(x: i32, y: i32, width: GlSizei, height: GlSizei) -> Self {
        Self {
            x,
            y,
            width: width.max(0),
            height: height.max(0),
            enabled: false,
        }
    }
}

impl Enabelable for Scissor {
    fn disable(&mut self) {
        self.enabled = false;
    }
    fn enable(&mut self) {
        self.enabled = true;
    }
    fn get_state(&self) -> bool {
        self.enabled
    }
}
