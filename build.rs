use std::env;
use std::process::Command;

fn main() {
    // Check if we're in a Nix environment 
    if env::var("NIX_STORE").is_ok() {
        println!("cargo:warning=Building in Nix environment - using system dependencies");
        return;
    }

    // Try to find required libraries
    let libraries = ["x11", "xrandr", "xext", "xfixes", "xcb", "xcb-shm"];
    
    for lib in &libraries {
        if !check_library_exists(lib) {
            println!("cargo:warning=Missing library: {}", lib);
            print_install_instructions();
            // Don't fail the build - let the linker handle it
        } else {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
}

fn check_library_exists(lib: &str) -> bool {
    Command::new("pkg-config")
        .args(&["--exists", lib])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn print_install_instructions() {
    println!("cargo:warning=");
    println!("cargo:warning=Missing system dependencies!");
    println!("cargo:warning=");
    println!("cargo:warning=For Fedora/Bazzite run:");
    println!("cargo:warning=sudo dnf install -y libX11-devel libXrandr-devel libXext-devel libXfixes-devel libxcb-devel");
    println!("cargo:warning=");
    println!("cargo:warning=For Ubuntu/Debian run:");
    println!("cargo:warning=sudo apt install -y libx11-dev libxrandr-dev libxext-dev libxfixes-dev libxcb1-dev");
    println!("cargo:warning=");
}