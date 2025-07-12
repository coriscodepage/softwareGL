pub static GL_MAX_COLOR_ATTACHMENTS: usize = 16;

#[repr(u32)]
pub enum ClearBufferMask {
    COLOR = 0x4000,
    DEPTH = 0x100,
    STENCIL = 0x400,
}

pub(crate) enum Framebuffer {
    Default,          // Known as the Default Framebuffer
    UserDefined(u32), //FBO
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum FramebufferTypes {
    DrawFramebuffer = 0x8ca9,
    ReadFramebuffer = 0x8ca8,
    Framebuffer = 0x8d40,
}

impl FramebufferTypes {
    pub(crate) fn from_u32(value: u32) -> Option<Self> {
        match value {
            n if Self::DrawFramebuffer as u32 == n => Some(Self::DrawFramebuffer),
            n if Self::ReadFramebuffer as u32 == n => Some(Self::ReadFramebuffer),
            n if Self::Framebuffer as u32 == n => Some(Self::Framebuffer),
            _ => None,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum DrawBufferSys {
    NONE = 0x0,
    Front = 0x404,      // Alias of FRONT_LEFT
    Back = 0x405,       // Alias of BACK_LEFT
    FrontLeft = 0x400,
    FrontRight = 0x401, // Not implemented
    BackLeft = 0x402,
    BackRight = 0x403,  // Not implemented
}

impl DrawBufferSys {
    pub(crate) fn from_u32(value: u32) -> Option<Self> {
        match value {
            n if Self::NONE as u32 == n => Some(Self::NONE),
            n if Self::Front as u32 == n => Some(Self::Front),
            n if Self::Back as u32 == n => Some(Self::Back),
            n if Self::FrontLeft as u32 == n => Some(Self::FrontLeft),
            n if Self::FrontRight as u32 == n => Some(Self::FrontRight),
            n if Self::BackLeft as u32 == n => Some(Self::BackLeft),
            n if Self::BackRight as u32 == n => Some(Self::BackRight),
            _ => None,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum DrawBufferFBO {
    None = 0x0,
    ColorAttachment0 = 0x8ce0,
    ColorAttachment1 = 0x8ce1,
    ColorAttachment2 = 0x8ce2,
    ColorAttachment3 = 0x8ce3,
    ColorAttachment4 = 0x8ce4,
    ColorAttachment5 = 0x8ce5,
    ColorAttachment6 = 0x8ce6,
    ColorAttachment7 = 0x8ce7,
    ColorAttachment8 = 0x8ce8,
    ColorAttachment9 = 0x8ce9,
    ColorAttachment10 = 0x8cea,
    ColorAttachment11 = 0x8ceb,
    ColorAttachment12 = 0x8cec,
    ColorAttachment13 = 0x8ced,
    ColorAttachment14 = 0x8cee,
    ColorAttachment15 = 0x8cef,
}

impl DrawBufferFBO {
    pub(crate) fn from_u32(value: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::None),
            n if Self::ColorAttachment0 as u32 == n => Some(Self::ColorAttachment0),
            n if Self::ColorAttachment1 as u32 == n => Some(Self::ColorAttachment1),
            n if Self::ColorAttachment2 as u32 == n => Some(Self::ColorAttachment2),
            n if Self::ColorAttachment3 as u32 == n => Some(Self::ColorAttachment3),
            n if Self::ColorAttachment4 as u32 == n => Some(Self::ColorAttachment4),
            n if Self::ColorAttachment5 as u32 == n => Some(Self::ColorAttachment5),
            n if Self::ColorAttachment6 as u32 == n => Some(Self::ColorAttachment6),
            n if Self::ColorAttachment7 as u32 == n => Some(Self::ColorAttachment7),
            n if Self::ColorAttachment8 as u32 == n => Some(Self::ColorAttachment8),
            n if Self::ColorAttachment9 as u32 == n => Some(Self::ColorAttachment9),
            n if Self::ColorAttachment10 as u32  == n=> Some(Self::ColorAttachment10),
            n if Self::ColorAttachment11 as u32  == n=> Some(Self::ColorAttachment11),
            n if Self::ColorAttachment12 as u32  == n=> Some(Self::ColorAttachment12),
            n if Self::ColorAttachment13 as u32  == n=> Some(Self::ColorAttachment13),
            n if Self::ColorAttachment14 as u32  == n=> Some(Self::ColorAttachment14),
            n if Self::ColorAttachment15 as u32  == n=> Some(Self::ColorAttachment15),
            _ => None,
        }
    }
    
    fn is_color_attachment(&self) -> bool {
        !matches!(self, Self::None)
    }
    
    pub(crate) fn get_attachment_index(&self) -> usize {
        match self {
            Self::None => usize::MAX,
            Self::ColorAttachment0 => 0,
            Self::ColorAttachment1 => 1,
            Self::ColorAttachment2 => 2,
            Self::ColorAttachment3 => 3,
            Self::ColorAttachment4 => 4,
            Self::ColorAttachment5 => 5,
            Self::ColorAttachment6 => 6,
            Self::ColorAttachment7 => 7,
            Self::ColorAttachment8 => 8,
            Self::ColorAttachment9 => 9,
            Self::ColorAttachment10 => 10,
            Self::ColorAttachment11 => 11,
            Self::ColorAttachment12 => 12,
            Self::ColorAttachment13 => 13,
            Self::ColorAttachment14 => 14,
            Self::ColorAttachment15 => 15,
        }
    }
}
