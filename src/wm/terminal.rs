//! Emulador de Terminal con estilo Unix moderno y comandos auténticos

use crate::graphics::color::*;
use crate::graphics::screen::Screen;
use crate::hardware::keyboard::KeyEvent;

pub struct Terminal {
    input_buf: [char; 32],
    input_len: usize,
    history: [&'static str; 8],
    history_count: usize,
}

impl Terminal {
    pub const fn new() -> Self {
        Self {
            input_buf: ['\0'; 32],
            input_len: 0,
            history: [
                "Linux-like Kernel ZoniOS 0.2.0 (x86_64)",
                "Escribe 'help' o 'ls' para explorar el sistema.",
                "",
                "",
                "",
                "",
                "",
                "",
            ],
            history_count: 2,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key {
            KeyEvent::Char(c) => {
                if self.input_len < 30 {
                    self.input_buf[self.input_len] = c;
                    self.input_len += 1;
                }
            }
            KeyEvent::Backspace => {
                if self.input_len > 0 {
                    self.input_len -= 1;
                    self.input_buf[self.input_len] = '\0';
                }
            }
            KeyEvent::Enter => {
                self.execute_command();
            }
        }
    }

    fn execute_command(&mut self) {
        let input = &self.input_buf[0..self.input_len];

        if self.is_cmd(input, "help") {
            self.add_history("Comandos Unix: ls, ps, free, uname, cat, clear, reboot");
        } else if self.is_cmd(input, "ls") {
            self.add_history("📁 bin/   📁 dev/   📁 etc/   📁 home/   📁 sys/");
        } else if self.is_cmd(input, "ps") {
            self.add_history("PID  CMD             STATUS   MEM");
            self.add_history("1    kernel_init     RUNNING  64K");
            self.add_history("2    compositor_wm   RUNNING  128K");
            self.add_history("3    ps2_daemon      IDLE     16K");
        } else if self.is_cmd(input, "free") {
            self.add_history("              total        used        free");
            self.add_history("Mem (RAM):    512M         64M         448M");
        } else if self.is_cmd(input, "uname") || self.is_cmd(input, "uname -a") {
            self.add_history("ZoniOS 0.2.0-generic x86_64 Rust Bare-Metal");
        } else if self.is_cmd(input, "cat /etc/os-release") || self.is_cmd(input, "cat os-release") {
            self.add_history("NAME=\"ZoniOS\"");
            self.add_history("PRETTY_NAME=\"ZoniOS Modern Unix Desktop\"");
            self.add_history("ARCH=\"x86_64\"");
        } else if self.is_cmd(input, "neofetch") || self.is_cmd(input, "fetch") {
            self.add_history("  /\\_/\\    OS: ZoniOS x86_64 (Rust)");
            self.add_history(" ( o.o )   WM: ZoniOS Top-Bar Compositor");
            self.add_history("  > ^ <    Uptime: Activo y fluido");
        } else if self.is_cmd(input, "clear") || self.is_cmd(input, "cls") {
            self.history = ["", "", "", "", "", "", "", ""];
            self.history_count = 0;
        } else if self.is_cmd(input, "reboot") || self.is_cmd(input, "poweroff") {
            self.add_history("Apagando / Reiniciando kernel...");
            unsafe {
                crate::hardware::port::outb(0x64, 0xFE);
            }
        } else if self.input_len == 0 {
            // Enter vacío
        } else {
            self.add_history("bash: comando no encontrado ('help')");
        }

        self.input_len = 0;
        for c in self.input_buf.iter_mut() {
            *c = '\0';
        }
    }

    fn is_cmd(&self, input: &[char], cmd: &str) -> bool {
        if input.len() != cmd.len() {
            return false;
        }
        for (i, c) in cmd.chars().enumerate() {
            if input[i] != c {
                return false;
            }
        }
        true
    }

    fn add_history(&mut self, line: &'static str) {
        if self.history_count < 8 {
            self.history[self.history_count] = line;
            self.history_count += 1;
        } else {
            for i in 0..7 {
                self.history[i] = self.history[i + 1];
            }
            self.history[7] = line;
        }
    }

    pub fn render(&self, screen: &mut Screen, x: usize, y: usize, w: usize, h: usize) {
        // Fondo negro profundo de terminal moderna (Alacritty / Kitty)
        let term_bg = Color::new(10, 14, 20);
        screen.draw_rect(x + 2, y + 29, w - 4, h - 31, term_bg);

        let mut text_y = y + 34;

        // Historial
        for i in 0..self.history_count {
            if !self.history[i].is_empty() {
                // Si la línea contiene '/', es un directorio (color cian moderno)
                let text_col = if self.history[i].contains('/') {
                    BOTON_INICIO
                } else {
                    TEXTO_MUTED
                };
                screen.draw_string(x + 10, text_y, self.history[i], text_col);
                text_y += 18;
            }
        }

        // Prompt moderno estilo Linux (verde + cian)
        let user = "zonios";
        let arrow = " ➜ ~ $ ";
        screen.draw_string(x + 10, text_y, user, BTN_MAXIMIZAR);
        screen.draw_string(x + 10 + (user.len() * 9), text_y, arrow, BOTON_INICIO);

        let prompt_len = user.len() + arrow.len();
        let mut cur_x = x + 10 + (prompt_len * 9);

        // Texto escrito por el usuario
        for i in 0..self.input_len {
            screen.draw_char(cur_x, text_y, self.input_buf[i], TEXTO_BLANCO);
            cur_x += 9;
        }

        // Cursor estilo bloque blanco
        screen.draw_rect(cur_x, text_y + 1, 8, 14, BLANCO);
    }
}
