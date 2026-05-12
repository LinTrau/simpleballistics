{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # 使用 crane 处理 Rust 构建
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;

        # 这里的 src 指定了源代码路径
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        # 构建依赖项（为了缓存）
        cargoArtifacts = craneLib.buildDepsOnly { inherit src; };

        # 构建主程序
        my-app = craneLib.buildPackage {
          inherit src cargoArtifacts;
          # 如果有非 Rust 的依赖（如 openssl, pkg-config）写在这里
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };
      in
      {
        packages.default = my-app;
        devShells.default = pkgs.mkShell {
          inputsFrom = [ my-app ];
          # 开发时额外需要的工具
          packages = [
            pkgs.rust-analyzer
            pkgs.cargo
          ];
        };
      }
    );
}
