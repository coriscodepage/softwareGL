use crate::enums::Framebuffer;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ColorValue {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl ColorValue {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ClearState {
    pub color_clear_value: ColorValue,
    pub depth_clear_value: f32,
    pub stenctil_clear_value: u8,
}


pub(crate) struct FramebufferState {
    pub read_framebuffer: Framebuffer,
    pub write_framebuffer: Framebuffer,
}