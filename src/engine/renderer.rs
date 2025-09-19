use anyhow::Result;
use pixels::{wgpu, Pixels, PixelsBuilder, SurfaceTexture};
use pixels::wgpu::{Backend, Backends, BlendState, DeviceDescriptor, TextureFormat};
use winit::window::CursorIcon::Default;
use winit::window::Window;
use zxcmath::Vector2;
use crate::engine::RendererConfigure;
use crate::graphics::{Color, Sprite};

pub struct RendererSystem {
    pixels: Pixels,
    width: u32,
    height: u32,
}

impl RendererSystem {
    pub(super) fn new(window: &Window, width: u32, height: u32, configure: RendererConfigure) -> Result<Self> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        let pixels = PixelsBuilder::new(width, height, surface_texture)
            .blend_state(configure.blend_state)
            .clear_color(configure.clear_color)
            .enable_vsync(configure.vsync)
            .wgpu_backend(configure.wgpu_backend)
            .build()?;

        Ok(Self {
            pixels,
            width,
            height,
        })
    }
    pub(super) fn clear_screen(&mut self, color: &[u8; 4]) {
        let frame = self.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(color);
        }
    }
    pub fn draw_rect(&mut self, position: &Vector2, size: &Vector2, color: &Color) {
        let color = color.unpack();
        let x_start = position.x as i32;
        let y_start = position.y as i32;
        let x_end = (position.x + size.x) as i32;
        let y_end = (position.y + size.y) as i32;
        let frame = self.pixels.frame_mut();

        for y in y_start..y_end {
            for x in x_start..x_end {
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    let index = (y as u32 * self.width + x as u32) as usize * 4;

                    if index + 3 < frame.len() {
                        frame[index..index + 4].copy_from_slice(&color);
                    }
                }
            }
        }
    }

    pub fn draw_sprite(&mut self, position: &Vector2, sprite: &Sprite) {
        let frame = self.pixels.frame_mut();
        let y_start = position.y as u32;
        let x_start = position.x as u32;
        let mut ci = 0;
        let size_x = sprite.width + x_start;
        let size_y = sprite.height + y_start;
        for y in y_start..size_y {
            for x in x_start..size_x {
                let index = (y * self.width + x) as usize * 4;
                if index + 3 < frame.len() {
                    frame[index..index + 4].copy_from_slice(&sprite.rgba[ci].unpack());
                }
                ci += 1;
            }
        }
    }
    
    pub fn draw_circle(&mut self, position: &Vector2, radius: f64, color: &Color) {
        let color = color.unpack();
        let x_start = (position.x - radius) as i32;
        let y_start = (position.y - radius) as i32;
        let x_end = (position.x + radius) as i32;
        let y_end = (position.y + radius) as i32;
        let frame = self.pixels.frame_mut();

        for y in y_start..y_end {
            for x in x_start..x_end {
                let dx = x as f64 - position.x;
                let dy = y as f64 - position.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= radius && x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    let index = (y as u32 * self.width + x as u32) as usize * 4;

                    if index + 3 < frame.len() {
                        frame[index..index + 4].copy_from_slice(&color);
                    }
                }
            }
        }
    }
    
    pub fn draw_line(&mut self, position_a: &Vector2, position_b: &Vector2, color: &Color) {
        let color = color.unpack();
        let x0 = position_a.x as i32;
        let y0 = position_a.y as i32;
        let x1 = position_b.x as i32;
        let y1 = position_b.y as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let step_x = if x0 < x1 { 1 } else { -1 };
        let step_y = if y0 < y1 { 1 } else { -1 };

        let mut error = if dx > dy { 2 * dy - dx } else { 2 * dx - dy };

        let mut x = x0;
        let mut y = y0;
        let frame = self.pixels.frame_mut();

        for _ in 0..= if dx >= dy { dx } else {dy} {
            let index = (y as u32 * self.width + x as u32) as usize * 4;
            if index + 3 < frame.len() {
                frame[index..index + 4].copy_from_slice(&color);
            }

            if error >= 0 {
                y += step_y;
                error -= 2 * dx;
            }
            error += 2 * dy;
            x += step_x;
        }
    }

    pub fn render(&mut self) -> Result<()> {
        self.pixels.render()?;
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.pixels.resize_surface(width, height)?;
        Ok(())
    }
}