//! Driver de teclado PS/2 para ZoniOS

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyEvent {
    Char(char),
    Enter,
    Backspace,
}

/// Traduce el código de escaneo (Scancode Set 1) a una tecla
pub fn scancode_to_key(scancode: u8) -> Option<KeyEvent> {
    // Si el bit 7 está activo (>= 0x80), es que la tecla se soltó (Break code)
    if scancode >= 0x80 {
        return None;
    }

    match scancode {
        // Letras
        0x1E => Some(KeyEvent::Char('a')),
        0x30 => Some(KeyEvent::Char('b')),
        0x2E => Some(KeyEvent::Char('c')),
        0x20 => Some(KeyEvent::Char('d')),
        0x12 => Some(KeyEvent::Char('e')),
        0x21 => Some(KeyEvent::Char('f')),
        0x22 => Some(KeyEvent::Char('g')),
        0x23 => Some(KeyEvent::Char('h')),
        0x17 => Some(KeyEvent::Char('i')),
        0x24 => Some(KeyEvent::Char('j')),
        0x25 => Some(KeyEvent::Char('k')),
        0x26 => Some(KeyEvent::Char('l')),
        0x32 => Some(KeyEvent::Char('m')),
        0x31 => Some(KeyEvent::Char('n')),
        0x18 => Some(KeyEvent::Char('o')),
        0x19 => Some(KeyEvent::Char('p')),
        0x10 => Some(KeyEvent::Char('q')),
        0x13 => Some(KeyEvent::Char('r')),
        0x1F => Some(KeyEvent::Char('s')),
        0x14 => Some(KeyEvent::Char('t')),
        0x16 => Some(KeyEvent::Char('u')),
        0x2F => Some(KeyEvent::Char('v')),
        0x11 => Some(KeyEvent::Char('w')),
        0x2D => Some(KeyEvent::Char('x')),
        0x15 => Some(KeyEvent::Char('y')),
        0x2C => Some(KeyEvent::Char('z')),

        // Números
        0x02 => Some(KeyEvent::Char('1')),
        0x03 => Some(KeyEvent::Char('2')),
        0x04 => Some(KeyEvent::Char('3')),
        0x05 => Some(KeyEvent::Char('4')),
        0x06 => Some(KeyEvent::Char('5')),
        0x07 => Some(KeyEvent::Char('6')),
        0x08 => Some(KeyEvent::Char('7')),
        0x09 => Some(KeyEvent::Char('8')),
        0x0A => Some(KeyEvent::Char('9')),
        0x0B => Some(KeyEvent::Char('0')),

        // Especiales
        0x39 => Some(KeyEvent::Char(' ')),
        0x34 => Some(KeyEvent::Char('.')),
        0x0C => Some(KeyEvent::Char('-')),
        0x1C => Some(KeyEvent::Enter),
        0x0E => Some(KeyEvent::Backspace),
        _ => None,
    }
}
