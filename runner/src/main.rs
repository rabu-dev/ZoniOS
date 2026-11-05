use std::process::Command;
use std::path::Path;

fn main() {
    println!("=== 1. Compilando el kernel ZoniOS ===");
    let status = Command::new("cargo")
        .args(&["build", "--package", "ZoniOS"])
        .status()
        .expect("Fallo al compilar el kernel");

    if !status.success() {
        std::process::exit(1);
    }

    println!("=== 2. Creando la imagen de disco arrancable ===");
    let kernel_path = Path::new("target/x86_64-unknown-none/debug/ZoniOS");
    let img_path = Path::new("target/zonios.img");

    bootloader::BiosBoot::new(kernel_path)
        .create_disk_image(img_path)
        .expect("Fallo al crear la imagen de disco con bootloader");

    println!("=== 3. Arrancando QEMU con ZoniOS ===");
    let qemu_path = r"C:\Program Files\qemu\qemu-system-x86_64.exe";
    let mut cmd = Command::new(qemu_path);
    cmd.args(&["-drive", "format=raw,file=target/zonios.img"]);
    
    let status = cmd.status().expect("Fallo al ejecutar QEMU");
    println!("QEMU cerrado con estado: {:?}", status);
}
