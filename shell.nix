{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    wayland
    wayland-protocols
    pipewire
    clang
    libclang
    libGL
    mesa
    libxkbcommon
    
    # X11/XCB Bibliotheken für enigo/scrap
    xorg.libxcb
    xorg.libX11
    xorg.libXrandr
    xorg.libXext
    xorg.libXfixes
    xdotool
  ];
  
  shellHook = ''
    export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
      pkgs.libGL
      pkgs.mesa
      pkgs.wayland
      pkgs.libxkbcommon
      pkgs.xorg.libxcb
      pkgs.xorg.libX11
      pkgs.xorg.libXrandr
      pkgs.xorg.libXext
      pkgs.xorg.libXfixes
      pkgs.xdotool
    ]}:$LD_LIBRARY_PATH"
    '';
}