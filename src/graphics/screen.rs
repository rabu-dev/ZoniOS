#![allow(dead_code)]
//! Abstracción de pantalla, framebuffer y primitivas de dibujo

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight};
use super::color::{Color, BLANCO, NEGRO};
use crate::hardware::mouse::{CURSOR_H, CURSOR_W, MOUSE_CURSOR};

pub struct Screen<'a> {
    buffer: &'a mut [u8],
    stride: usize,
    bytes_per_pixel: usize,
    pixel_format: PixelFormat,
    pub width: usize,
    pub height: usize,
}

impl<'a> Screen<'a> {
    pub fn new(buffer: &'a mut [u8], info: FrameBufferInfo) -> Self {
        Self {
            buffer,
            stride: info.stride,
            bytes_per_pixel: info.bytes_per_pixel,
            pixel_format: info.pixel_format,
            width: info.width,
            height: info.height,
        }
    }

    #[inline(always)]
    pub fn put_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.stride + x) * self.bytes_per_pixel;
        if offset + self.bytes_per_pixel <= self.buffer.len() {
            match self.pixel_format {
                PixelFormat::Rgb => {
                    self.buffer[offset]     = color.r;
                    self.buffer[offset + 1] = color.g;
                    self.buffer[offset + 2] = color.b;
                }
                PixelFormat::Bgr => {
                    self.buffer[offset]     = color.b;
                    self.buffer[offset + 1] = color.g;
                    self.buffer[offset + 2] = color.r;
                }
                PixelFormat::U8 => {
                    self.buffer[offset]     = color.r;
                }
                _ => {}
            }
        }
    }

    #[inline(always)]
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x >= self.width || y >= self.height {
            return NEGRO;
        }
        let offset = (y * self.stride + x) * self.bytes_per_pixel;
        if offset + self.bytes_per_pixel <= self.buffer.len() {
            match self.pixel_format {
                PixelFormat::Rgb => Color::new(
                    self.buffer[offset],
                    self.buffer[offset + 1],
                    self.buffer[offset + 2],
                ),
                PixelFormat::Bgr => Color::new(
                    self.buffer[offset + 2],
                    self.buffer[offset + 1],
                    self.buffer[offset],
                ),
                _ => NEGRO,
            }
        } else {
            NEGRO
        }
    }

    #[inline(always)]
    pub fn clear(&mut self, color: Color) {
        self.draw_rect(0, 0, self.width, self.height, color);
    }

    #[inline(always)]
    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        let max_x = (x + w).min(self.width);
        let max_y = (y + h).min(self.height);

        for cur_y in y..max_y {
            let row_start = cur_y * self.stride;
            for cur_x in x..max_x {
                let offset = (row_start + cur_x) * self.bytes_per_pixel;
                if offset + self.bytes_per_pixel <= self.buffer.len() {
                    match self.pixel_format {
                        PixelFormat::Rgb => {
                            self.buffer[offset]     = color.r;
                            self.buffer[offset + 1] = color.g;
                            self.buffer[offset + 2] = color.b;
                        }
                        PixelFormat::Bgr => {
                            self.buffer[offset]     = color.b;
                            self.buffer[offset + 1] = color.g;
                            self.buffer[offset + 2] = color.r;
                        }
                        PixelFormat::U8 => {
                            self.buffer[offset]     = color.r;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn draw_gradient_v(&mut self, top: Color, bottom: Color) {
        for y in 0..self.height {
            let factor = (y * 256) / self.height;
            let r = ((top.r as usize * (256 - factor)) + (bottom.r as usize * factor)) / 256;
            let g = ((top.g as usize * (256 - factor)) + (bottom.g as usize * factor)) / 256;
            let b = ((top.b as usize * (256 - factor)) + (bottom.b as usize * factor)) / 256;
            let row_color = Color::new(r as u8, g as u8, b as u8);
            self.draw_rect(0, y, self.width, 1, row_color);
        }
    }

    pub fn draw_char(&mut self, x: usize, y: usize, c: char, color: Color) {
        if let Some(bitmap) = get_raster(c, FontWeight::Regular, RasterHeight::Size16) {
            for (row_idx, row) in bitmap.rows().enumerate() {
                for (col_idx, &intensity) in row.iter().enumerate() {
                    if intensity > 128 {
                        self.put_pixel(x + col_idx, y + row_idx, color);
                    }
                }
            }
        }
    }

    pub fn draw_string(&mut self, mut x: usize, y: usize, text: &str, color: Color) {
        for c in text.chars() {
            self.draw_char(x, y, c, color);
            x += 9;
        }
    }

    pub fn save_cursor_bg(&self, mouse_x: usize, mouse_y: usize, bg: &mut [Color; CURSOR_W * CURSOR_H]) {
        for r in 0..CURSOR_H {
            for c in 0..CURSOR_W {
                bg[r * CURSOR_W + c] = self.get_pixel(mouse_x + c, mouse_y + r);
            }
        }
    }

    pub fn restore_cursor_bg(&mut self, mouse_x: usize, mouse_y: usize, bg: &[Color; CURSOR_W * CURSOR_H]) {
        for r in 0..CURSOR_H {
            for c in 0..CURSOR_W {
                self.put_pixel(mouse_x + c, mouse_y + r, bg[r * CURSOR_W + c]);
            }
        }
    }

    pub fn draw_cursor(&mut self, mouse_x: usize, mouse_y: usize) {
        for (row, line) in MOUSE_CURSOR.iter().enumerate() {
            for (col, char_byte) in line.bytes().enumerate() {
                match char_byte {
                    b'B' => self.put_pixel(mouse_x + col, mouse_y + row, NEGRO),
                    b'W' => self.put_pixel(mouse_x + col, mouse_y + row, BLANCO),
                    _ => {}
                }
            }
        }
    }
}
