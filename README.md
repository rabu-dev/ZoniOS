# 🚀 ZoniOS

> Un sistema operativo de 64 bits desarrollado desde cero en **Rust** (Bare-Metal, `no_std`), con un entorno de escritorio moderno inspirado en Linux/Unix, gestor de ventanas, compositor gráfico de alta definición (1280x720) y controladores de hardware en tiempo real.

---

## ✨ Características

- 🦀 **Rust Puro Bare-Metal (`no_std`)**: Sin dependencias de Windows ni Linux. Ejecución directa sobre el procesador x86_64.
- 🎬 **Pantalla de carga (Splash Screen)**: Arranque animado con barra de progreso y fases de inicialización de hardware.
- 🖥️ **Compositor Gráfico HD (1280x720)**:
  - Renderizado directo en **Hardware GOP Framebuffer** a 60 FPS estables.
  - Paleta moderna estilo *Dark Mode / Nord*.
  - Ventanas flotantes con **sombras suaves** y controles de ventana estilo semáforo (🔴 🟡 🟢).
  - Fondo degradado vertical de alta precisión.
- 🖱️ **Driver de Ratón PS/2 por Hardware**:
  - Lectura de puertos I/O (`0x60` y `0x64`) con ensamblador en línea.
  - Movimiento suave y arrastre de ventanas (*Drag & Drop*) en tiempo real.
  - Cursor con mapa de bits y restauración de fondo sin parpadeos.
- ⌨️ **Driver de Teclado PS/2**:
  - Traducción de Scancodes Set 1.
  - Entrada de texto en vivo para la terminal.
- 💻 **Terminal Interactiva Unix-like**:
  - Prompt estilo moderno: `zonios ➜ ~ $ `.
  - Comandos disponibles: `ls`, `ps`, `free`, `uname -a`, `cat /etc/os-release`, `neofetch`, `clear`, `reboot`.
- 📁 **Explorador de Archivos y Monitor**:
  - Pestaña `Archivos`: Vista de directorios raíz Unix (`/bin`, `/dev`, `/home`, `/etc`).
  - Pestaña `Monitor`: Medidores gráficos de CPU y memoria RAM estilo `htop`.
- ⏰ **Reloj RTC por Hardware**:
  - Lee la hora real de la placa base (chip Motorola MC146818) a través de los puertos `0x70` y `0x71`.
- 🚀 **Top-Bar estilo GNOME**:
  - Barra superior con botón de aplicaciones, reloj central y widgets de estado.
  - Menú desplegable para abrir/cerrar ventanas o reiniciar.

---

## 🏛️ Arquitectura del Kernel

```text
src/
├── main.rs                   <- Punto de entrada del kernel y orquestador
│
├── hardware/                 <- Capa 0: Hardware y periféricos
│   ├── port.rs               <- Instrucciones I/O x86 ('inb' y 'outb')
│   ├── mouse.rs              <- Driver de ratón PS/2 (Intel 8042)
│   ├── keyboard.rs           <- Driver de teclado PS/2 y traducción de Scancodes
│   └── rtc.rs                <- Reloj en tiempo real (Motorola MC146818)
│
├── graphics/                 <- Capa 1: Renderizado de vídeo
│   ├── color.rs              <- Paleta moderna Dark Mode
│   ├── screen.rs             <- Abstracción del Framebuffer, degradados y texto
│   └── splash.rs             <- Pantalla de carga animada (Splash Screen)
│
└── wm/                       <- Capa 2: Gestor de Ventanas y Escritorio
    ├── window.rs             <- Struct Window, colisiones y arrastre
    ├── desktop.rs            <- Compositor de la Top-Bar y Menú
    ├── terminal.rs           <- Emulador de Terminal y shell interactiva
    └── system_info.rs        <- Explorador de Archivos y Monitor de Sistema
```

---

## 🛠️ Requisitos

- **Rust Nightly** con target `x86_64-unknown-none`:
  ```bash
  rustup default nightly
  rustup target add x86_64-unknown-none
  rustup component add llvm-tools-preview rust-src
  ```
- **QEMU** para emulación x86_64 (`qemu-system-x86_64`).

---

## 🚀 Cómo Ejecutar

En PowerShell:

```powershell
.\run.ps1
```

El script compilará el kernel en modo optimizado, empaquetará la imagen de disco arrancable con el bootloader y abrirá la máquina virtual QEMU.
