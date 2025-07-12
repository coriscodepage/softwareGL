use crate::{enums::Framebuffer, types::ColorValue};


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