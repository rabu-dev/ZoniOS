//! Compositor con Barra Superior moderna estilo Linux (GNOME / Top-Bar)

use crate::graphics::color::*;
use crate::graphics::screen::Screen;
use super::window::Window;
use super::terminal::Terminal;
use super::system_info::SystemInfoView;

pub fn redraw_desktop(
    screen: &mut Screen,
    win1: &Window,
    sys_info: &SystemInfoView,
    win2: &Window,
    terminal: &Terminal,
    win3: &Window,
    menu_abierto: bool,
    time_str: &str,
) {
    // 1. Fondo de pantalla degradado moderno (Pizarra profunda)
    screen.draw_gradient_v(FONDO_TOP, FONDO_BOTTOM);

    // 2. Ventana "Archivos" (Unix File Manager)
    if win1.visible {
        win1.render(screen);
        sys_info.render(screen, win1.x, win1.y, win1.width, win1.height);
    }

    // 3. Ventana "Terminal ZoniOS"
    if win2.visible {
        win2.render(screen);
        terminal.render(screen, win2.x, win2.y, win2.width, win2.height);
    }

    // 4. Ventana "Ajustes de ZoniOS"
    if win3.visible {
        win3.render(screen);
        let inner_bg = Color::new(20, 24, 33);
        screen.draw_rect(win3.x + 2, win3.y + 29, win3.width - 4, win3.height - 31, inner_bg);

        screen.draw_string(win3.x + 14, win3.y + 40, "Configuracion de ZoniOS", TEXTO_BLANCO);
        screen.draw_rect(win3.x + 14, win3.y + 58, win3.width - 28, 1, BORDE_VENTANA);

        screen.draw_string(win3.x + 14, win3.y + 70, "Entorno: ZoniOS Modern Unix Desktop", BOTON_INICIO);
        screen.draw_string(win3.x + 14, win3.y + 92, "Shell: zonios-sh (POSIX compatible)", BTN_MAXIMIZAR);
        screen.draw_string(win3.x + 14, win3.y + 114, "Top-Bar: GNOME/Hyprland Inspired", TEXTO_MUTED);
        screen.draw_string(win3.x + 14, win3.y + 136, "Hardware: Reloj RTC + PS/2 Activo", TEXTO_MUTED);
    }

    // 5. BARRA SUPERIOR (TOP-BAR ESTILO LINUX GNOME)
    let bar_h = 34;
    screen.draw_rect(0, 0, screen.width, bar_h, BARRA_TAREAS);
    screen.draw_rect(0, bar_h - 1, screen.width, 1, BORDE_VENTANA); // Borde inferior fino

    // 5.1 Botón Izquierdo: [ 🚀 Aplicaciones ]
    screen.draw_rect(8, 4, 120, 26, BOTON_INICIO);
    screen.draw_string(16, 9, "🚀 Aplicaciones", BLANCO);

    // 5.2 Centro: Reloj RTC en pastilla elegante
    let cx = screen.width / 2;
    screen.draw_rect(cx - 40, 4, 80, 26, RELOJ_FONDO);
    screen.draw_string(cx - 24, 9, time_str, TEXTO_BLANCO);

    // 5.3 Derecha: Pastillas de estado del sistema (RAM, CPU, Apagar)
    // Pastilla RAM
    screen.draw_rect(screen.width - 210, 4, 90, 26, RELOJ_FONDO);
    screen.draw_string(screen.width - 200, 9, "RAM 64M", BTN_MAXIMIZAR);

    // Pastilla CPU
    screen.draw_rect(screen.width - 114, 4, 70, 26, RELOJ_FONDO);
    screen.draw_string(screen.width - 104, 9, "60 FPS", BOTON_INICIO);

    // Botón Apagado
    screen.draw_rect(screen.width - 38, 4, 30, 26, SOMBRA);
    screen.draw_string(screen.width - 30, 9, "⏻", BTN_CERRAR);

    // 6. MENÚ DE APLICACIONES (Se despliega hacia abajo desde la barra superior)
    if menu_abierto {
        let menu_x = 8;
        let menu_y = bar_h + 4; // Cae justo debajo de la barra superior
        let menu_w = 200;
        let menu_h = 210;

        // Sombra flotante
        screen.draw_rect(menu_x + 4, menu_y + 4, menu_w, menu_h, SOMBRA);

        // Contenedor del menú
        screen.draw_rect(menu_x, menu_y, menu_w, menu_h, VENTANA_BG);
        screen.draw_rect(menu_x, menu_y, menu_w, 1, BORDE_VENTANA);
        screen.draw_rect(menu_x, menu_y, 1, menu_h, BORDE_VENTANA);
        screen.draw_rect(menu_x + menu_w - 1, menu_y, 1, menu_h, BORDE_VENTANA);
        screen.draw_rect(menu_x, menu_y + menu_h - 1, menu_w, 1, BORDE_VENTANA);

        // Encabezado
        screen.draw_string(menu_x + 16, menu_y + 14, "ZoniOS Applications", BOTON_INICIO);
        screen.draw_rect(menu_x + 14, menu_y + 36, menu_w - 28, 1, BORDE_VENTANA);

        // Opciones
        screen.draw_string(menu_x + 16, menu_y + 52, "📁 Archivos (Files)", TEXTO_BLANCO);
        screen.draw_string(menu_x + 16, menu_y + 82, "💻 Terminal (Bash)", TEXTO_BLANCO);
        screen.draw_string(menu_x + 16, menu_y + 112, "⚙ Ajustes (Settings)", TEXTO_BLANCO);
        screen.draw_rect(menu_x + 14, menu_y + 142, menu_w - 28, 1, BORDE_VENTANA);
        screen.draw_string(menu_x + 16, menu_y + 162, "⏻ Apagar / Reboot", BTN_CERRAR);
    }
}
