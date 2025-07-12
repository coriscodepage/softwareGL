use std::{mem, sync::{Arc, Mutex}};

use crate::{context::{with_current_context, GLSharedState, GlContext, GLOBAL_STATE}, types::GlBool};


#[unsafe(no_mangle)]
pub extern "C" fn glKCreateContext(width: usize, height: usize, share_with: usize, double_buffered: GlBool) -> usize {
    let context = GlContext::init(width, height, double_buffered);
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
pub extern "C" fn glKSwapBuffers() {
    with_current_context(|context| {
        mem::swap(&mut context.default_framebuffer.color_buffer_back, &mut context.default_framebuffer.color_buffer_front);
    });
}