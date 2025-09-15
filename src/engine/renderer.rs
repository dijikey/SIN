use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;
use zxcmath::Vector2;
use crate::graphics::Color;

pub struct RendererSystem {
    pixels: Pixels,
    width: u32,
    height: u32,
}

impl RendererSystem {
    pub(super) fn new(window: &Window, width: u32, height: u32) -> Result<Self> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        let pixels = Pixels::new(width, height, surface_texture)?;

        Ok(Self {
            pixels,
            width,
            height,
        })
    }

    pub fn clear(&mut self, color: Color) {
        let frame = self.pixels.frame_mut();
        let color = color.unpack();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }

    pub fn draw_rect(&mut self, position: Vector2, size: Vector2, color: Color) {
        let color = color.unpack();
        let x_start = position.x as i32;
        let y_start = position.y as i32;
        let x_end = (position.x + size.x) as i32;
        let y_end = (position.y + size.y) as i32;

        for y in y_start..y_end {
            for x in x_start..x_end {
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    let index = (y as u32 * self.width + x as u32) as usize * 4;
                    let frame = self.pixels.frame_mut();
                    if index + 3 < frame.len() {
                        frame[index..index + 4].copy_from_slice(&color);
                    }
                }
            }
        }
    }
    
    pub fn draw_line(&mut self, a: Vector2, b: Vector2, color: Color) {
        let color = color.unpack();
        let x0 = a.x as i32;
        let y0 = a.y as i32;
        let x1 = b.x as i32;
        let y1 = b.y as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let step_x = if x0 < x1 { 1 } else { -1 };
        let step_y = if y0 < y1 { 1 } else { -1 };

        let mut error = if dx > dy { 2 * dy - dx } else { 2 * dx - dy };

        let mut x = x0;
        let mut y = y0;
        
        if dx >= dy {
            for _ in 0..=dx {
                let index = (y as u32 * self.width + x as u32) as usize * 4;
                let frame = self.pixels.frame_mut();
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
        } else {
            for _ in 0..=dy {
                let index = (y as u32 * self.width + x as u32) as usize * 4;
                let frame = self.pixels.frame_mut();
                if index + 3 < frame.len() {
                    frame[index..index + 4].copy_from_slice(&color);
                }

                if error >= 0 {
                    x += step_x;
                    error -= 2 * dy;
                }
                error += 2 * dx;
                y += step_y;
            }
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