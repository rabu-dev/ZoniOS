#![allow(dead_code)]
//! Definición de color y paleta moderna Dark Mode

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

// Colores básicos
pub const NEGRO: Color        = Color::new(0, 0, 0);
pub const BLANCO: Color       = Color::new(255, 255, 255);

// Paleta moderna estilo Dark Mode / Pizarra
pub const FONDO_TOP: Color    = Color::new(15, 23, 42);    // Azul noche profundo
pub const FONDO_BOTTOM: Color = Color::new(30, 41, 59);    // Pizarra oscura
pub const SOMBRA: Color       = Color::new(8, 10, 18);     // Sombra flotante
pub const VENTANA_BG: Color   = Color::new(24, 28, 36);    // Cuerpo de ventana
pub const BARRA_TITULO: Color = Color::new(32, 38, 50);    // Barra de ventana
pub const BORDE_VENTANA: Color= Color::new(60, 70, 90);    // Borde fino elegante
pub const TEXTO_BLANCO: Color = Color::new(240, 243, 246);
pub const TEXTO_MUTED: Color  = Color::new(140, 150, 170);
pub const BARRA_TAREAS: Color = Color::new(18, 22, 30);    // Barra de tareas
pub const BOTON_INICIO: Color = Color::new(59, 130, 246);  // Azul eléctrico
pub const RELOJ_FONDO: Color  = Color::new(28, 34, 46);

// Botones de control de ventana (Semáforo)
pub const BTN_CERRAR: Color   = Color::new(239, 68, 68);   // Rojo
pub const BTN_MINIMIZAR: Color= Color::new(234, 179, 8);   // Amarillo
pub const BTN_MAXIMIZAR: Color= Color::new(34, 197, 94);   // Verde
