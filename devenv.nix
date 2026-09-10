{
  pkgs,
  lib,
  ...
}:

let
  systemPackages = with pkgs; [
    openssl
    pkg-config
  ];

  cliTools = with pkgs; [
    ripgrep
    fd
    sd
    jaq
    ast-grep
  ];
in
{
  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  languages.javascript = {
    enable = true;
    bun.enable = true;
    bun.install.enable = true;
  };

  packages =
    (with pkgs; [
      just
      sqlx-cli
      cargo-expand
      mkcert
      kubectl
      kubernetes-helm
      kind
      terraform
      gh
      nixfmt
    ])
    ++ cliTools
    ++ systemPackages
    ++ lib.optionals pkgs.stdenv.hostPlatform.isDarwin [ pkgs.libiconv ];

  env.SQLX_OFFLINE = "true";

  enterShell = ''
    echo "rust $(rustc --version | cut -d' ' -f2) · bun $(bun --version)" >&2
    echo "run 'just' to list recipes" >&2
  '';

  enterTest = ''
    rustc --version
    bun --version
    just --version
  '';
}
