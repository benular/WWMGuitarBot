# Installation Requirements

## System Dependencies

This project requires several system libraries that must be installed before building.

### Ubuntu/Debian
```bash
sudo apt update
sudo apt install -y \
    pkg-config \
    libwayland-dev \
    libxkbcommon-dev \
    libgl1-mesa-dev \
    libx11-dev \
    libxrandr-dev \
    libxext-dev \
    libxfixes-dev \
    libxcb1-dev \
    libxdamage-dev
```

### Fedora/RHEL/CentOS
```bash
sudo dnf install -y \
    pkg-config \
    wayland-devel \
    libxkbcommon-devel \
    mesa-libGL-devel \
    libX11-devel \
    libXrandr-devel \
    libXext-devel \
    libXfixes-devel \
    libxcb-devel
```

### Arch Linux
```bash
sudo pacman -S \
    pkg-config \
    wayland \
    libxkbcommon \
    mesa \
    libx11 \
    libxrandr \
    libxext \
    libxfixes \
    libxcb
```

### macOS
```bash
# Most dependencies should work out of the box
# If issues occur, install via Homebrew:
brew install pkg-config
```

### Windows
- Visual Studio Build Tools or Visual Studio Community
- Windows SDK
- Most dependencies are statically linked

## Building
```bash
git clone <repository>
cd chinge_bot
cargo build --release
```

## Runtime Requirements

### Wayland
- Requires compositor that supports screencopy protocol (wlroots-based)
- Examples: Sway, River, Hyprland

### X11
- Should work on most X11 setups
- Requires XTEST extension (usually available)

## Troubleshooting

### "pkg-config not found"
Install pkg-config for your distribution (see above)

### "library not found" errors
Ensure all development packages are installed for your distribution

### Wayland permission issues
Some compositors require additional permissions for screen capture