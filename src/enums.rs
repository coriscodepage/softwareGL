#[repr(u32)]
pub enum ClearBufferMask {
    COLOR = 0x4000,
    DEPTH = 0x100,
    STENCIL = 0x400,
}

pub(crate) enum Framebuffer {
    Default,
    UserDefined(u32),
}
