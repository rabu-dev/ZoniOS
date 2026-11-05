//! Estructura de Ventana y lógica de interacción

use crate::graphics::color::*;
use crate::graphics::screen::Screen;

pub struct Window {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub title: &'static str,
    pub visible: bool,
}

impl Window {
    pub const fn new(x: usize, y: usize, width: usize, height: usize, title: &'static str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            title,
            visible: true,
        }
    }

    pub fn render(&self, screen: &mut Screen) {
        if !self.visible {
            return;
        }

        // 1. Sombra suave
        screen.draw_rect(self.x + 5, self.y + 5, self.width, self.height, SOMBRA);

        // 2. Fondo de la ventana
        screen.draw_rect(self.x, self.y, self.width, self.height, VENTANA_BG);

        // 3. Barra de título integrada
        screen.draw_rect(self.x, self.y, self.width, 28, BARRA_TITULO);

        // 4. Bordes sutiles
        screen.draw_rect(self.x, self.y, self.width, 1, BORDE_VENTANA);
        screen.draw_rect(self.x, self.y, 1, self.height, BORDE_VENTANA);
        screen.draw_rect(self.x + self.width - 1, self.y, 1, self.height, BORDE_VENTANA);
        screen.draw_rect(self.x, self.y + self.height - 1, self.width, 1, BORDE_VENTANA);

        // 5. Los 3 botones modernos (🔴 🟡 🟢)
        screen.draw_rect(self.x + 10, self.y + 8, 12, 12, BTN_CERRAR);
        screen.draw_rect(self.x + 28, self.y + 8, 12, 12, BTN_MINIMIZAR);
        screen.draw_rect(self.x + 46, self.y + 8, 12, 12, BTN_MAXIMIZAR);

        // 6. Título
        screen.draw_string(self.x + 68, self.y + 7, self.title, TEXTO_BLANCO);
    }

    pub fn is_over_close_button(&self, mx: usize, my: usize) -> bool {
        if !self.visible {
            return false;
        }
        let btn_x = self.x + 10;
        let btn_y = self.y + 8;
        mx >= btn_x && mx <= btn_x + 14 && my >= btn_y && my <= btn_y + 14
    }

    pub fn is_over_title_bar(&self, mx: usize, my: usize) -> bool {
        if !self.visible {
            return false;
        }
        mx >= (self.x + 60) && mx <= (self.x + self.width) && my >= self.y && my <= (self.y + 28)
    }
}
