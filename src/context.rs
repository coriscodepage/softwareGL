use std::{collections::HashMap, sync::{Arc, LazyLock, Mutex}};

use crate::{enums::Framebuffer, states::{ClearState, FramebufferState}, types::{DefaultFramebuffer, GlBool, Scissor, FBO}};


pub(crate) struct GlobalState {
    pub contexts: HashMap<usize, GlContext>,
    pub next_context_id: usize,
    pub current_context: usize,
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

pub(crate) static GLOBAL_STATE: LazyLock<Mutex<GlobalState>> =
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
pub(crate) struct GLSharedState {
    //pub textures: RwLock<HashMap<>>
}
impl GLSharedState {
    pub(crate) fn init() -> Self {
        Self {}
    }
}

pub struct GlContext {
    pub shared: Option<Arc<Mutex<GLSharedState>>>,
    pub clear_state: ClearState,
    pub next_fb_id: u32,
    pub framebuffer_objects: HashMap<u32, FBO>,
    pub default_framebuffer: DefaultFramebuffer,
    pub framebuffer_state: FramebufferState,
    pub viewport: Viewport,
    pub scissor: Scissor,
}

impl GlContext {
    pub fn init(width: usize, height: usize, double_buffered: GlBool) -> Self {
        let framebuffer_objects = HashMap::with_capacity(1);
        let system_fb = DefaultFramebuffer::init(width, height, double_buffered);
        Self {
            shared: None,
            clear_state: ClearState::default(),
            next_fb_id: 1,
            framebuffer_objects,
            default_framebuffer: system_fb,
            framebuffer_state: FramebufferState {
                read_framebuffer: Framebuffer::Default,
                write_framebuffer: Framebuffer::Default,
            },
            viewport: Viewport::default(),
            scissor: Scissor::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Viewport {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}