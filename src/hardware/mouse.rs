//! Controlador de periféricos PS/2 (Intel 8042: Teclado y Ratón)

use super::port::{inb, outb, wait_read, wait_write};

pub const CURSOR_W: usize = 10;
pub const CURSOR_H: usize = 13;

pub const MOUSE_CURSOR: [&str; CURSOR_H] = [
    "B.........",
    "BB........",
    "BWB.......",
    "BWWB......",
    "BWWWB.....",
    "BWWWWB....",
    "BWWWWWB...",
    "BWWWWWWB..",
    "BWWWWBBB..",
    "BWBWWB....",
    "BB..BWWB..",
    ".....BWWB.",
    "......BB..",
];

#[allow(dead_code)]
pub struct MouseEvent {
    pub dx: i16,
    pub dy: i16,
    pub left_click: bool,
    pub right_click: bool,
}

pub struct MouseDriver {
    packet_cycle: u8,
    bytes: [u8; 3],
}

impl MouseDriver {
    pub const fn new() -> Self {
        Self {
            packet_cycle: 0,
            bytes: [0; 3],
        }
    }

    /// Inicializa el chip 8042 activando tanto el teclado como el ratón
    pub unsafe fn init(&self) {
        unsafe {
            // 1. Activar el puerto del Teclado (0xAE) y del Ratón (0xA8)
            wait_write();
            outb(0x64, 0xAE); // Activa el puerto de teclado
            wait_write();
            outb(0x64, 0xA8); // Activa el puerto auxiliar de ratón

            // 2. Leer y configurar el byte de estado del controlador 8042
            wait_write();
            outb(0x64, 0x20); // Comando: Leer byte de configuración
            wait_read();
            let mut config = inb(0x60);

            config |= 0x01;  // Bit 0: Habilita interrupción de teclado
            config |= 0x02;  // Bit 1: Habilita interrupción de ratón
            config |= 0x40;  // Bit 6: Habilita traducción Scancode Set 2 -> Set 1
            config &= !0x10; // Bit 4: Desactiva el bloqueo del reloj de teclado (0 = habilitado)
            config &= !0x20; // Bit 5: Desactiva el bloqueo del reloj de ratón (0 = habilitado)

            wait_write();
            outb(0x64, 0x60); // Comando: Escribir byte de configuración
            wait_write();
            outb(0x60, config);

            // 3. Activar reporte de datos en el ratón
            wait_write();
            outb(0x64, 0xD4);
            wait_write();
            outb(0x60, 0xF6); // Valores por defecto
            let _ = wait_read();
            let _ = inb(0x60); // ACK

            wait_write();
            outb(0x64, 0xD4);
            wait_write();
            outb(0x60, 0xF4); // Habilitar data reporting continuo
            let _ = wait_read();
            let _ = inb(0x60); // ACK

            // 4. Limpiar cualquier byte residual que haya quedado en el buffer
            while (inb(0x64) & 0x01) != 0 {
                let _ = inb(0x60);
            }
        }
    }

    /// Procesa un byte procedente del ratón
    pub fn handle_byte(&mut self, data: u8) -> Option<MouseEvent> {
        match self.packet_cycle {
            0 => {
                if (data & 0x08) != 0 {
                    self.bytes[0] = data;
                    self.packet_cycle = 1;
                }
            }
            1 => {
                self.bytes[1] = data;
                self.packet_cycle = 2;
            }
            2 => {
                self.bytes[2] = data;
                self.packet_cycle = 0;

                let flags = self.bytes[0];
                let left_click = (flags & 0x01) != 0;
                let right_click = (flags & 0x02) != 0;

                let mut dx = self.bytes[1] as i16;
                if (flags & 0x10) != 0 {
                    dx |= !0xFF;
                }

                let mut dy = self.bytes[2] as i16;
                if (flags & 0x20) != 0 {
                    dy |= !0xFF;
                }

                return Some(MouseEvent {
                    dx,
                    dy,
                    left_click,
                    right_click,
                });
            }
            _ => self.packet_cycle = 0,
        }
        None
    }
}
