use std::slice;

use crate::{
    context::with_current_context,
    enums::{
        ClearBufferMask, DrawBufferFBO, DrawBufferSys, Framebuffer, FramebufferTypes,
        GL_MAX_COLOR_ATTACHMENTS,
    },
    types::{self, ColorValue, FBO, GlBitfield, GlSizei},
};

#[unsafe(no_mangle)]
pub extern "C" fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32) {
    with_current_context(|context| {
        context.clear_state.color_clear_value = ColorValue::new(red, green, blue, alpha);
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn glClear(mask: types::GlBitfield) {
    with_current_context(
        |context| match context.framebuffer_state.write_framebuffer {
            Framebuffer::Default => match mask {
                n if ClearBufferMask::COLOR as u32 & n != 0 => {
                    match context.default_framebuffer.draw_buffer {
                        crate::enums::DrawBufferSys::Front
                        | crate::enums::DrawBufferSys::FrontLeft => {
                            context
                                .default_framebuffer
                                .color_buffer_front
                                .pixels
                                .fill(context.clear_state.color_clear_value);
                        }
                        crate::enums::DrawBufferSys::Back
                        | crate::enums::DrawBufferSys::BackLeft => {
                            context
                                .default_framebuffer
                                .color_buffer_back
                                .pixels
                                .fill(context.clear_state.color_clear_value);
                        }
                        crate::enums::DrawBufferSys::NONE => {}
                        _ => panic!("Unimplemented"),
                    }
                }
                n if ClearBufferMask::DEPTH as u32 & n != 0 => {
                    if let Some(depth_buffer) = &mut context.default_framebuffer.depth_buffer {
                        depth_buffer
                            .pixels
                            .fill(context.clear_state.depth_clear_value);
                    }
                }
                n if ClearBufferMask::STENCIL as u32 & n != 0 => {
                    if let Some(stencil_buffer) = &mut context.default_framebuffer.stencil_buffer {
                        stencil_buffer
                            .pixels
                            .fill(context.clear_state.stenctil_clear_value);
                    }
                }
                _ => panic!(),
            },
            Framebuffer::UserDefined(fbo_id) => {
                let active_framebuffer = context.framebuffer_objects.get_mut(&fbo_id).unwrap();
                for buf in active_framebuffer.draw_buffers.iter() {
                    if let Some(buffer) = buf {
                        let index = buffer.get_attachment_index() as usize;
                        active_framebuffer.color_attachments[index]
                            .pixels
                            .fill(context.clear_state.color_clear_value);
                    }
                }
            }
        },
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn glDrawBuffers(n: i32, bufs: *mut u32) {
    if n < 0 {
        return; // TODO: GL_ERROR GL_INVALID_VALUE
    }
    if n == 0 {
        return;
    }
    if bufs.is_null() {
        return; // TODO: GL_ERROR GL_INVALID_VALUE
    }
    let bufs_slice = unsafe { slice::from_raw_parts(bufs, n as usize) };
    with_current_context(|context| {
        match &context.framebuffer_state.write_framebuffer {
            Framebuffer::Default => {
                if n > 1 {
                    panic!("Only one draw buffer can be bound to the default Framebuffer"); // TODO: GL_ERROR
                }
                for &buf in bufs_slice {
                    match DrawBufferSys::from_u32(buf) {
                        Some(DrawBufferSys::Back) | Some(DrawBufferSys::BackLeft) => {
                            context.default_framebuffer.draw_buffer = DrawBufferSys::BackLeft;
                        }
                        Some(DrawBufferSys::Front) | Some(DrawBufferSys::FrontLeft) => {
                            context.default_framebuffer.draw_buffer = DrawBufferSys::FrontLeft;
                        }
                        Some(DrawBufferSys::NONE) => {
                            context.default_framebuffer.draw_buffer = DrawBufferSys::NONE;
                        }
                        _ => panic!("Unknown Enum"), // TODO: GL_ERROR
                    }
                }
            }
            Framebuffer::UserDefined(fbo_id) => {
                if n as usize > GL_MAX_COLOR_ATTACHMENTS {
                    return; // TODO: GL_ERROR GL_INVALID_VALUE
                }
                let current_fbo = context.framebuffer_objects.get_mut(fbo_id).unwrap();
                current_fbo.draw_buffers.fill(None);
                for &buf in bufs_slice.iter() {
                    match DrawBufferFBO::from_u32(buf) {
                        Some(DrawBufferFBO::None) => {}
                        Some(indexed_attachment) => {
                            let index = indexed_attachment.get_attachment_index();
                            current_fbo.draw_buffers[index] = Some(indexed_attachment);
                        }
                        _ => todo!(),
                    }
                }
            }
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn glNamedFramebufferDrawBuffers(framebuffer: u32, n: i32, bufs: *const u32) {
    if n < 0 {
        return; // TODO: GL_ERROR GL_INVALID_VALUE
    }
    if n == 0 {
        return;
    }
    if bufs.is_null() {
        return; // TODO: GL_ERROR GL_INVALID_VALUE
    }
    if n as usize > GL_MAX_COLOR_ATTACHMENTS {
        return; // TODO: GL_ERROR GL_INVALID_VALUE
    }
    let bufs_slice = unsafe { slice::from_raw_parts(bufs, n as usize) };

    with_current_context(|context| {
        if let Some(fbo) = context.framebuffer_objects.get_mut(&framebuffer) {
            fbo.draw_buffers.fill(None);
            for &buf in bufs_slice.iter() {
                match DrawBufferFBO::from_u32(buf) {
                    Some(DrawBufferFBO::None) => {}
                    Some(indexed_attachment) => {
                        let index = indexed_attachment.get_attachment_index();
                        fbo.draw_buffers[index] = Some(indexed_attachment);
                    }
                    _ => todo!(),
                }
            }
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn glDrawBuffer(buf: u32) {
    with_current_context(|context| {
        match &context.framebuffer_state.write_framebuffer {
            Framebuffer::Default => {
                match DrawBufferSys::from_u32(buf) {
                    Some(DrawBufferSys::Back) | Some(DrawBufferSys::BackLeft) => {
                        context.default_framebuffer.draw_buffer = DrawBufferSys::BackLeft;
                    }
                    Some(DrawBufferSys::Front) | Some(DrawBufferSys::FrontLeft) => {
                        context.default_framebuffer.draw_buffer = DrawBufferSys::FrontLeft;
                    }
                    Some(DrawBufferSys::NONE) => {
                        context.default_framebuffer.draw_buffer = DrawBufferSys::NONE;
                    }
                    _ => panic!("Unknown Enum"), // TODO: GL_ERROR
                }
            }
            Framebuffer::UserDefined(fbo_id) => {
                
                let current_fbo = context.framebuffer_objects.get_mut(fbo_id).unwrap();
                current_fbo.draw_buffers.fill(None);
                match DrawBufferFBO::from_u32(buf) {
                    Some(DrawBufferFBO::None) => {}
                    Some(indexed_attachment) => {
                        let index = indexed_attachment.get_attachment_index();
                        current_fbo.draw_buffers[index] = Some(indexed_attachment);
                    }
                    _ => todo!(),
                }
            }
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn glNamedFramebufferDrawBuffer(framebuffer: u32, buf: u32) {
    with_current_context(|context| {
        if let Some(fbo) = context.framebuffer_objects.get_mut(&framebuffer) {
            fbo.draw_buffers.fill(None);
            match DrawBufferFBO::from_u32(buf) {
                Some(DrawBufferFBO::None) => {}
                Some(indexed_attachment) => {
                    let index = indexed_attachment.get_attachment_index();
                    fbo.draw_buffers[index] = Some(indexed_attachment);
                }
                _ => todo!(),
            }
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn glGenFramebuffers(n: GlSizei, ids: *mut u32) {
    let framebuffer_ids = unsafe { slice::from_raw_parts_mut(ids, n as usize) };
    with_current_context(|context| {
        for i in 0..n as usize {
            let key = context.next_fb_id;
            framebuffer_ids[i] = key;
            let framebuffer = FBO::new(
                context.default_framebuffer.width,
                context.default_framebuffer.height,
            );
            context.framebuffer_objects.insert(key, framebuffer);
            context.next_fb_id += 1;
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn glDeleteFramebuffers(n: GlSizei, framebuffers: *const u32) {
    let framebuffer_ids = unsafe { slice::from_raw_parts(framebuffers, n as usize) };
    with_current_context(|context| {
        for i in 0..n as usize {
            let key = &framebuffer_ids[i];
            context.framebuffer_objects.remove(key);
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn glBindFramebuffer(target: u32, framebuffer: u32) {
    if target == 0 {
        with_current_context(|context| match FramebufferTypes::from_u32(framebuffer) {
            Some(FramebufferTypes::DrawFramebuffer) => {
                context.framebuffer_state.write_framebuffer = Framebuffer::Default
            }
            Some(FramebufferTypes::ReadFramebuffer) => {
                context.framebuffer_state.read_framebuffer = Framebuffer::Default
            }
            Some(FramebufferTypes::Framebuffer) => {
                context.framebuffer_state.read_framebuffer = Framebuffer::Default;
                context.framebuffer_state.write_framebuffer = Framebuffer::Default
            }
            _ => panic!(),
        })
    } else {
        with_current_context(|context| match FramebufferTypes::from_u32(framebuffer) {
            Some(FramebufferTypes::DrawFramebuffer) => {
                context.framebuffer_state.write_framebuffer = Framebuffer::UserDefined(target)
            }
            Some(FramebufferTypes::ReadFramebuffer) => {
                context.framebuffer_state.read_framebuffer = Framebuffer::UserDefined(target)
            }
            Some(FramebufferTypes::Framebuffer) => {
                context.framebuffer_state.read_framebuffer = Framebuffer::UserDefined(target);
                context.framebuffer_state.write_framebuffer = Framebuffer::UserDefined(target)
            }
            None => panic!(),
        });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn glBlitFramebuffer(
    srcX0: i32,
    srcY0: i32,
    srcX1: i32,
    srcY1: i32,
    dstX0: i32,
    dstY0: i32,
    dstX1: i32,
    dstY1: i32,
    mask: GlBitfield,
    filter: u32,
) {
    todo!()
}

#[unsafe(no_mangle)]
pub extern "C" fn glBlitNamedFramebuffer(
    readFramebuffer: u32,
    drawFramebuffer: u32,
    srcX0: i32,
    srcY0: i32,
    srcX1: i32,
    srcY1: i32,
    dstX0: i32,
    dstY0: i32,
    dstX1: i32,
    dstY1: i32,
    mask: GlBitfield,
    filter: u32,
) {
    todo!()
}
