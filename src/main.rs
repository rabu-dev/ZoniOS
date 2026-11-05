#![no_std]
#![no_main]

mod hardware;
mod graphics;
mod wm;

use core::panic::PanicInfo;
use bootloader_api::entry_point;
use bootloader_api::BootInfo;

use hardware::mouse::{MouseDriver, CURSOR_H, CURSOR_W};
use hardware::keyboard::scancode_to_key;
use hardware::port::{inb, outb};
use hardware::rtc::read_rtc_time;
use graphics::color::NEGRO;
use graphics::screen::Screen;
use graphics::splash::show_splash_screen;
use wm::window::Window;
use wm::desktop::redraw_desktop;
use wm::terminal::Terminal;
use wm::system_info::SystemInfoView;

// Solicitar resolución panorámica de alta definición HD (1280x720)
#[allow(deprecated)]
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.frame_buffer.minimum_framebuffer_width = Some(1280);
    config.frame_buffer.minimum_framebuffer_height = Some(720);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn format_time(buf: &mut [u8; 5]) -> &str {
    let time = read_rtc_time();
    buf[0] = b'0' + (time.hours / 10);
    buf[1] = b'0' + (time.hours % 10);
    buf[2] = b':';
    buf[3] = b'0' + (time.minutes / 10);
    buf[4] = b'0' + (time.minutes % 10);
    core::str::from_utf8(buf).unwrap_or("12:00")
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let mut screen = Screen::new(framebuffer.buffer_mut(), info);

        // 1. Splash Screen de arranque
        show_splash_screen(&mut screen);

        // 2. Ventanas con estética Linux / Unix
        let mut ventana1 = Window::new(60, 55, 380, 250, "Archivos");
        let mut ventana2 = Window::new(260, 95, 420, 270, "Terminal ZoniOS");
        let mut mut_win3 = Window::new(160, 70, 360, 220, "Ajustes de ZoniOS");
        mut_win3.visible = false;

        // Aplicaciones funcionales
        let mut terminal = Terminal::new();
        let mut sys_info = SystemInfoView::new();

        let mut menu_abierto = false;

        let mut time_buf = [b'0'; 5];
        let time_str = format_time(&mut time_buf);

        // 3. Render inicial del escritorio con la Top-Bar moderna
        redraw_desktop(&mut screen, &ventana1, &sys_info, &ventana2, &terminal, &mut_win3, menu_abierto, time_str);

        // 4. Cursor del ratón centrado
        let mut mouse_x = screen.width / 2;
        let mut mouse_y = screen.height / 2;
        let mut bg_buffer = [NEGRO; CURSOR_W * CURSOR_H];

        screen.save_cursor_bg(mouse_x, mouse_y, &mut bg_buffer);
        screen.draw_cursor(mouse_x, mouse_y);

        // 5. Inicializar hardware (teclado + ratón)
        let mut mouse = MouseDriver::new();
        unsafe { mouse.init(); }

        let mut dragging_win: Option<u8> = None;
        let mut prev_left_click = false;

        // --- BUCLE PRINCIPAL DE EVENTOS DEL KERNEL ---
        loop {
            let status = unsafe { inb(0x64) };
            if (status & 0x01) != 0 {
                let data = unsafe { inb(0x60) };

                let mut needs_redraw = false;

                // --- ¿ES DEL RATÓN? ---
                if (status & 0x20) != 0 {
                    if let Some(event) = mouse.handle_byte(data) {
                        let left_click = event.left_click;
                        let dx = event.dx;
                        let dy = event.dy;

                        let new_x = (mouse_x as i32 + dx as i32)
                            .clamp(0, (screen.width - CURSOR_W) as i32) as usize;
                        let new_y = (mouse_y as i32 - dy as i32)
                            .clamp(0, (screen.height - CURSOR_H) as i32) as usize;

                        if left_click && !prev_left_click {
                            // Clic en el botón [ 🚀 Aplicaciones ] en la Top-Bar
                            if new_x >= 8 && new_x <= 128 && new_y >= 4 && new_y <= 30 {
                                menu_abierto = !menu_abierto;
                                needs_redraw = true;

                            // Clic en el botón de Apagado [⏻] de la esquina superior derecha
                            } else if new_x >= (screen.width - 38) && new_x <= (screen.width - 8) && new_y >= 4 && new_y <= 30 {
                                unsafe { outb(0x64, 0xFE); }

                            // Clic dentro del Menú de Aplicaciones (desplegado hacia abajo)
                            } else if menu_abierto && new_x >= 8 && new_x <= 208 {
                                let menu_y = 38;
                                if new_y >= menu_y + 46 && new_y <= menu_y + 72 {
                                    // 📁 Archivos
                                    ventana1.visible = true;
                                    menu_abierto = false;
                                    needs_redraw = true;
                                } else if new_y >= menu_y + 76 && new_y <= menu_y + 102 {
                                    // 💻 Terminal
                                    ventana2.visible = true;
                                    menu_abierto = false;
                                    needs_redraw = true;
                                } else if new_y >= menu_y + 106 && new_y <= menu_y + 132 {
                                    // ⚙ Ajustes
                                    mut_win3.visible = true;
                                    menu_abierto = false;
                                    needs_redraw = true;
                                } else if new_y >= menu_y + 154 && new_y <= menu_y + 184 {
                                    // ⏻ Apagar / Reiniciar
                                    unsafe { outb(0x64, 0xFE); }
                                }

                            // Botones de cerrar de las ventanas (🔴)
                            } else if mut_win3.is_over_close_button(new_x, new_y) {
                                mut_win3.visible = false;
                                needs_redraw = true;
                            } else if ventana2.is_over_close_button(new_x, new_y) {
                                ventana2.visible = false;
                                needs_redraw = true;
                            } else if ventana1.is_over_close_button(new_x, new_y) {
                                ventana1.visible = false;
                                needs_redraw = true;

                            // Barras de título para arrastrar
                            } else if mut_win3.is_over_title_bar(new_x, new_y) {
                                dragging_win = Some(3);
                            } else if ventana2.is_over_title_bar(new_x, new_y) {
                                dragging_win = Some(2);
                            } else if ventana1.is_over_title_bar(new_x, new_y) {
                                dragging_win = Some(1);

                            // Pestañas del Explorador de Archivos / Monitor
                            } else if ventana1.visible
                                && new_x >= ventana1.x && new_x <= (ventana1.x + ventana1.width)
                                && new_y >= ventana1.y && new_y <= (ventana1.y + ventana1.height)
                            {
                                let rel_x = new_x - ventana1.x;
                                let rel_y = new_y - ventana1.y;
                                if sys_info.handle_click(rel_x, rel_y) {
                                    needs_redraw = true;
                                }
                            }
                        }

                        if !left_click {
                            dragging_win = None;
                        }

                        // Arrastrar ventana activa
                        if let Some(target) = dragging_win {
                            if dx != 0 || dy != 0 {
                                if target == 3 {
                                    mut_win3.x = (mut_win3.x as i32 + dx as i32).max(0) as usize;
                                    mut_win3.y = (mut_win3.y as i32 - dy as i32).max(36) as usize;
                                } else if target == 2 {
                                    ventana2.x = (ventana2.x as i32 + dx as i32).max(0) as usize;
                                    ventana2.y = (ventana2.y as i32 - dy as i32).max(36) as usize;
                                } else if target == 1 {
                                    ventana1.x = (ventana1.x as i32 + dx as i32).max(0) as usize;
                                    ventana1.y = (ventana1.y as i32 - dy as i32).max(36) as usize;
                                }
                                needs_redraw = true;
                            }
                        }

                        prev_left_click = left_click;

                        if needs_redraw {
                            let time_str = format_time(&mut time_buf);
                            redraw_desktop(&mut screen, &ventana1, &sys_info, &ventana2, &terminal, &mut_win3, menu_abierto, time_str);
                            mouse_x = new_x;
                            mouse_y = new_y;
                            screen.save_cursor_bg(mouse_x, mouse_y, &mut bg_buffer);
                            screen.draw_cursor(mouse_x, mouse_y);
                        } else if dx != 0 || dy != 0 {
                            screen.restore_cursor_bg(mouse_x, mouse_y, &bg_buffer);
                            mouse_x = new_x;
                            mouse_y = new_y;
                            screen.save_cursor_bg(mouse_x, mouse_y, &mut bg_buffer);
                            screen.draw_cursor(mouse_x, mouse_y);
                        }
                    }
                // --- ¿ES DEL TECLADO? ---
                } else {
                    if let Some(key) = scancode_to_key(data) {
                        terminal.handle_key(key);
                        needs_redraw = true;
                    }

                    if needs_redraw {
                        let time_str = format_time(&mut time_buf);
                        redraw_desktop(&mut screen, &ventana1, &sys_info, &ventana2, &terminal, &mut_win3, menu_abierto, time_str);
                        screen.save_cursor_bg(mouse_x, mouse_y, &mut bg_buffer);
                        screen.draw_cursor(mouse_x, mouse_y);
                    }
                }
            }
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
