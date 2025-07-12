use std::{thread, time::Duration};

use sdl3::{
    self,
    pixels::{PixelFormat, PixelFormatEnum},
};

use crate::{enums::ClearBufferMask, renderer::GlContext};
mod renderer;
mod states;
mod types;
mod enums;

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
    let mut gl_ctx = GlContext::init(WINDOW_WIDTH, WINDOW_HEIGHT);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl3::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        gl_ctx.gl_clear_color(1.0, 0.0, 0.0, 1.0);
        gl_ctx.gl_clear(ClearBufferMask::COLOR as u32);
        texture
            .update(None, gl_ctx.system_fb.as_slice_u8().as_slice(), (WINDOW_WIDTH * 4) as usize)
            .unwrap();
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
}
