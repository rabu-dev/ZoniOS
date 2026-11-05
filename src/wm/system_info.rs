//! Explorador de Archivos Unix y Monitor del Sistema

use crate::graphics::color::*;
use crate::graphics::screen::Screen;

pub struct SystemInfoView {
    pub selected_tab: u8, // 0 = Archivos, 1 = Monitor del Sistema
}

impl SystemInfoView {
    pub const fn new() -> Self {
        Self { selected_tab: 0 }
    }

    pub fn render(&self, screen: &mut Screen, x: usize, y: usize, w: usize, h: usize) {
        let inner_bg = Color::new(18, 22, 30);
        screen.draw_rect(x + 2, y + 29, w - 4, h - 31, inner_bg);

        // --- PESTAÑAS ESTILO NORD / GNOME ---
        let tab_w = 110;
        let tab1_col = if self.selected_tab == 0 { BOTON_INICIO } else { BARRA_TITULO };
        let tab2_col = if self.selected_tab == 1 { BOTON_INICIO } else { BARRA_TITULO };

        screen.draw_rect(x + 8, y + 35, tab_w, 24, tab1_col);
        screen.draw_string(x + 14, y + 40, "📁 Archivos", BLANCO);

        screen.draw_rect(x + 124, y + 35, tab_w, 24, tab2_col);
        screen.draw_string(x + 130, y + 40, "📊 Monitor", BLANCO);

        // Línea separadora
        screen.draw_rect(x + 8, y + 64, w - 16, 1, BORDE_VENTANA);

        if self.selected_tab == 0 {
            // EXPLORADOR DE ARCHIVOS UNIX
            screen.draw_string(x + 14, y + 74, "Ruta actual: / (Root File System)", TEXTO_MUTED);

            // Carpetas
            screen.draw_rect(x + 14, y + 96, 75, 42, SOMBRA);
            screen.draw_rect(x + 14, y + 96, 75, 1, BORDE_VENTANA);
            screen.draw_string(x + 22, y + 104, "📁 bin", BOTON_INICIO);
            screen.draw_string(x + 22, y + 122, "4 items", TEXTO_MUTED);

            screen.draw_rect(x + 98, y + 96, 75, 42, SOMBRA);
            screen.draw_rect(x + 98, y + 96, 75, 1, BORDE_VENTANA);
            screen.draw_string(x + 106, y + 104, "📁 dev", BOTON_INICIO);
            screen.draw_string(x + 106, y + 122, "3 devs", TEXTO_MUTED);

            screen.draw_rect(x + 182, y + 96, 75, 42, SOMBRA);
            screen.draw_rect(x + 182, y + 96, 75, 1, BORDE_VENTANA);
            screen.draw_string(x + 190, y + 104, "📁 home", BOTON_INICIO);
            screen.draw_string(x + 190, y + 122, "zonios", TEXTO_MUTED);

            screen.draw_rect(x + 266, y + 96, 75, 42, SOMBRA);
            screen.draw_rect(x + 266, y + 96, 75, 1, BORDE_VENTANA);
            screen.draw_string(x + 274, y + 104, "📁 etc", BOTON_INICIO);
            screen.draw_string(x + 274, y + 122, "config", TEXTO_MUTED);

            screen.draw_string(x + 14, y + 154, "Sistema montado en RAM (Initrd Bare-Metal)", TEXTO_MUTED);
        } else {
            // MONITOR DEL SISTEMA (HTOP STYLE)
            screen.draw_string(x + 14, y + 74, "Rendimiento del Sistema", TEXTO_BLANCO);

            // CPU
            screen.draw_string(x + 14, y + 96, "CPU [||||                 ] 12%", BOTON_INICIO);

            // RAM
            screen.draw_string(x + 14, y + 116, "RAM [||||||||             ] 64M / 512M", BTN_MAXIMIZAR);

            // Tareas
            screen.draw_string(x + 14, y + 138, "Tareas: 3 activas, 0 zombie", TEXTO_MUTED);
            screen.draw_string(x + 14, y + 158, "Carga: 0.02, 0.01 (x86_64 60Hz)", TEXTO_MUTED);
        }
    }

    pub fn handle_click(&mut self, rel_x: usize, rel_y: usize) -> bool {
        // Pestaña 1 (x: 8..118, y: 35..59)
        if rel_x >= 8 && rel_x <= 118 && rel_y >= 35 && rel_y <= 59 {
            self.selected_tab = 0;
            return true;
        }
        // Pestaña 2 (x: 124..234, y: 35..59)
        if rel_x >= 124 && rel_x <= 234 && rel_y >= 35 && rel_y <= 59 {
            self.selected_tab = 1;
            return true;
        }
        false
    }
}
