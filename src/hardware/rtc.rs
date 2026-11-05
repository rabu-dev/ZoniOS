//! Driver del Reloj en Tiempo Real (RTC / Motorola MC146818)
//! Lee la hora real de la placa base a través de los puertos 0x70 y 0x71

use super::port::{inb, outb};

#[allow(dead_code)]
pub struct SystemTime {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

/// Convierte formato BCD (Binary Coded Decimal) a binario estándar
fn bcd_to_bin(val: u8) -> u8 {
    ((val >> 4) * 10) + (val & 0x0F)
}

/// Lee la hora real exacta de la placa base
pub fn read_rtc_time() -> SystemTime {
    unsafe {
        // Registro 0x04: Horas
        outb(0x70, 0x04);
        let raw_hours = inb(0x71);

        // Registro 0x02: Minutos
        outb(0x70, 0x02);
        let raw_minutes = inb(0x71);

        // Registro 0x00: Segundos
        outb(0x70, 0x00);
        let raw_seconds = inb(0x71);

        SystemTime {
            hours: bcd_to_bin(raw_hours),
            minutes: bcd_to_bin(raw_minutes),
            seconds: bcd_to_bin(raw_seconds),
        }
    }
}
