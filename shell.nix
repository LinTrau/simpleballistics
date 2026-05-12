{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust 工具链
    cargo
    rustc
    rust-analyzer
    clippy

    julia-bin

    openblas
    pkg-config
  ];

  # 2. 环境变量配置
  shellHook = ''
    export RUST_BACKTRACE=1
  '';
}
