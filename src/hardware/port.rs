//! Instrucciones de bajo nivel de puertos I/O de la arquitectura x86

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

#[inline(always)]
pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Espera a que el buffer de entrada del chip PS/2 esté listo para recibir un comando
#[inline(always)]
pub unsafe fn wait_write() {
    let mut timeout = 100_000;
    while timeout > 0 && (unsafe { inb(0x64) } & 0x02) != 0 {
        timeout -= 1;
    }
}

/// Espera a que el buffer de salida del chip PS/2 tenga datos disponibles para leer
#[inline(always)]
pub unsafe fn wait_read() -> bool {
    let mut timeout = 100_000;
    while timeout > 0 {
        if (unsafe { inb(0x64) } & 0x01) != 0 {
            return true;
        }
        timeout -= 1;
    }
    false
}
