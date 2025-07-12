use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};

use crate::{
    enums::{ClearBufferMask, Framebuffer},
    states::{ClearState, ColorValue, FramebufferState},
    types,
};

static GL_MAX_COLOR_ATTACHMENTS: usize = 16;

struct GlobalState {
    contexts: HashMap<usize, GlContext>,
    next_context_id: usize,
    current_context: usize,
}

impl GlobalState {
    fn init() -> Self {
        Self {
            contexts: HashMap::new(),
            next_context_id: 1,
            current_context: 1,
        }
    }
}

static GLOBAL_STATE: LazyLock<Mutex<GlobalState>> =
    LazyLock::new(|| Mutex::new(GlobalState::init()));

pub fn with_current_context<F, R>(f: F) -> R
where
    F: FnOnce(&mut GlContext) -> R,
{
    let mut state = GLOBAL_STATE.lock().unwrap();
    let current = state.current_context;
    let ctx = state.contexts.get_mut(&current).unwrap();
    f(ctx)
}

// Shared state that can be shared between contexts.
struct GLSharedState {
    //textures: RwLock<HashMap<>>
}
impl GLSharedState {
    fn init() -> Self {
        Self {}
    }
}
struct ColorBuffer {
    width: usize,
    height: usize,
    pixels: Vec<ColorValue>,
}

impl ColorBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![ColorValue::default(); width * height],
        }
    }
}

struct DepthBuffer {
    width: usize,
    height: usize,
    pixels: Vec<f32>,
}

impl DepthBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0f32; width * height],
        }
    }
}

struct StencilBuffer {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

pub struct DepthStencilBuffer {
    pub width: usize,
    pub height: usize,
    pub depth: Vec<f32>,
    pub stencil: Vec<u8>,
}

struct FBO {
    width: usize,
    height: usize,
    color_attachments: [ColorBuffer; GL_MAX_COLOR_ATTACHMENTS],
    depth_attachment: Option<DepthBuffer>,
    stencil_attachment: Option<StencilBuffer>,
    depth_stencil_attachment: Option<DepthStencilBuffer>,
}

pub struct DefaultFramebuffer {
    width: usize,
    height: usize,
    color_buffer_front: ColorBuffer,
    color_buffer_back: ColorBuffer,
    depth_buffer: Option<DepthBuffer>,
    stencil_buffer: Option<StencilBuffer>,
}
impl DefaultFramebuffer {
    fn init(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color_buffer_front: ColorBuffer::new(width, height),
            color_buffer_back: ColorBuffer::new(width, height),
            depth_buffer: Some(DepthBuffer::new(width, height)),
            stencil_buffer: None,
        }
    }
    pub fn as_slice_u8(&self) -> Vec<u8> {
        let mut ret = Vec::<u8>::with_capacity(self.height * self.width * 4);
        for pixel in &self.color_buffer_back.pixels {
            ret.push((pixel.alpha * 255.0) as u8);
            ret.push((pixel.blue * 255.0) as u8);
            ret.push((pixel.green * 255.0) as u8);
            ret.push((pixel.red * 255.0) as u8);
        }
        ret
    }
}

pub struct GlContext {
    shared: Option<Arc<Mutex<GLSharedState>>>,
    clear_state: ClearState,
    next_fb_id: u32,
    framebuffer_objects: HashMap<u32, FBO>,
    pub system_fb: DefaultFramebuffer,
    framebuffer_state: FramebufferState,
}

impl GlContext {
    pub fn init(width: usize, height: usize) -> Self {
        let framebuffer_objects = HashMap::with_capacity(1);
        let system_fb = DefaultFramebuffer::init(width, height);
        Self {
            shared: None,
            clear_state: ClearState::default(),
            next_fb_id: 0,
            framebuffer_objects,
            system_fb,
            framebuffer_state: FramebufferState {
                read_framebuffer: Framebuffer::Default,
                write_framebuffer: Framebuffer::Default,
            },
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn glKCreateContext(width: usize, height: usize, share_with: usize) -> usize {
    let context = GlContext::init(width, height);
    let mut global_state = GLOBAL_STATE.lock().unwrap();
    let context_id = global_state.next_context_id;
    global_state.contexts.insert(context_id, context);
    global_state.next_context_id += 1;
    if share_with != 0 {
        if let Some(share_with_context) = global_state.contexts.get(&share_with) {
            with_current_context(|context| context.shared = Some(share_with_context.shared.clone().unwrap_or_else(|| Arc::new(Mutex::new(GLSharedState::init())))));
        }
    }
    context_id
}

#[unsafe(no_mangle)]
pub extern "C" fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32) {
    with_current_context(|context| {
        context.clear_state.color_clear_value = ColorValue::new(red, green, blue, alpha);
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn glClear(mask: types::Glbitfield) {
    with_current_context(
        |context| match context.framebuffer_state.write_framebuffer {
            Framebuffer::Default => match mask {
                n if ClearBufferMask::COLOR as u32 & n != 0 => {
                    context
                        .system_fb
                        .color_buffer_back
                        .pixels
                        .fill(context.clear_state.color_clear_value);
                }
                n if ClearBufferMask::DEPTH as u32 & n != 0 => {
                    if let Some(depth_buffer) = &mut context.system_fb.depth_buffer {
                        depth_buffer
                            .pixels
                            .fill(context.clear_state.depth_clear_value);
                    }
                }
                n if ClearBufferMask::STENCIL as u32 & n != 0 => {
                    if let Some(stencil_buffer) = &mut context.system_fb.stencil_buffer {
                        stencil_buffer
                            .pixels
                            .fill(context.clear_state.stenctil_clear_value);
                    }
                }
                _ => panic!(),
            },
            Framebuffer::UserDefined(fbo_id) => todo!(),
        },
    );
}
