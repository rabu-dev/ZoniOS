//! Pantalla de carga (Splash Screen) al arrancar ZoniOS

use super::color::*;
use super::screen::Screen;

fn delay(ticks: usize) {
    for _ in 0..ticks {
        core::hint::spin_loop();
    }
}

pub fn show_splash_screen(screen: &mut Screen) {
    // 1. Fondo oscuro elegante
    screen.clear(FONDO_TOP);

    let cx = screen.width / 2;
    let cy = screen.height / 2;

    // Título y subtítulo en el centro
    screen.draw_string(cx - 28, cy - 50, "ZoniOS", TEXTO_BLANCO);
    screen.draw_string(cx - 68, cy - 25, "Cargando el Kernel...", TEXTO_MUTED);

    // Dimensiones de la barra de progreso
    let bar_w = 220;
    let bar_h = 8;
    let bar_x = cx - (bar_w / 2);
    let bar_y = cy + 10;

    // Marco de la barra
    screen.draw_rect(bar_x - 2, bar_y - 2, bar_w + 4, bar_h + 4, SOMBRA);
    screen.draw_rect(bar_x - 1, bar_y - 1, bar_w + 2, bar_h + 2, BORDE_VENTANA);

    // Fases de arranque reales
    let pasos = [
        (25,  "Iniciando memoria RAM..."),
        (55,  "Configurando Framebuffer..."),
        (85,  "Activando controlador PS/2..."),
        (100, "Iniciando escritorio ZoniOS..."),
    ];

    for &(porcentaje, mensaje) in pasos.iter() {
        // Borramos el mensaje anterior y ponemos el nuevo
        screen.draw_rect(cx - 140, cy + 30, 280, 20, FONDO_TOP);
        screen.draw_string(cx - 100, cy + 32, mensaje, TEXTO_MUTED);

        // Llenamos la barra de carga
        let progreso_w = (bar_w * porcentaje) / 100;
        screen.draw_rect(bar_x, bar_y, progreso_w, bar_h, BOTON_INICIO);

        // Retardo visible
        delay(50_000_000);
    }

    // Pequeña pausa final antes del escritorio
    delay(40_000_000);
}
