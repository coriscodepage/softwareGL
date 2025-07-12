use std::{thread, time::Duration};

use sdl3::{
    self,
    pixels::{PixelFormat, PixelFormatEnum},
};

use crate::{context::with_current_context, enums::{ClearBufferMask, DrawBufferSys}, renderer::{glBindFramebuffer, glBlitFramebuffer, glClear, glClearColor, glDrawBuffer, glDrawBuffers, glGenFramebuffers}, KoriExt::{glKCreateContext, glKSwapBuffers}};
mod renderer;
mod states;
mod types;
mod enums;
#[allow(non_snake_case)]
mod KoriExt;
mod context;

fn main() {
    const WINDOW_WIDTH: usize = 800;
    const WINDOW_HEIGHT: usize = 600;
    let sdl_ctx = sdl3::init().unwrap();
    let video_sub = sdl_ctx.video().unwrap();
    let sdl_window = video_sub
        .window("Renderer", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .resizable()
        .position_centered()
        .build()
        .unwrap();
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    let mut canvas = sdl_window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            unsafe { PixelFormat::from_ll(PixelFormatEnum::RGBA8888.to_ll()) },
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
        )
        .unwrap();
    glKCreateContext(WINDOW_WIDTH, WINDOW_HEIGHT, 0, 1);
    //glDrawBuffers(1, [0x405u32; 1].as_mut_ptr());
    let mut cmask = 1u8;
    let mut test_fbo: [u32; 1] = [0u32];
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl3::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        glClearColor(1.0 * (cmask & 0x1) as f32, 1.0 * (cmask >> 1 & 0x1) as f32, 1.0 * (cmask >> 2 & 0x1) as f32, 1.0);
        cmask = cmask << 1;
        if cmask >= 8 {
            cmask = 1;
        }
        glClear(ClearBufferMask::COLOR as u32);
        glKSwapBuffers();
        texture
            .update(None, &with_current_context(|ctx| ctx.default_framebuffer.as_slice_u8(DrawBufferSys::FrontLeft)), (WINDOW_WIDTH * 4) as usize)
            .unwrap();
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        thread::sleep(Duration::from_millis(250));
    }
}
