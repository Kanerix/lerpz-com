{ pkgs, lib, config, inputs, ... }:

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
      mkcert
      kubectl
      kubernetes-helm
      kind
      terraform
    ])
    ++ cliTools
    ++ systemPackages
    ++ lib.optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];

  env.SQLX_OFFLINE = "true";

  enterShell = ''
    echo "rust $(rustc --version | cut -d' ' -f2) · bun $(bun --version)"
    echo "run 'just' to list recipes"
  '';

  enterTest = ''
    rustc --version
    bun --version
    just --version
  '';
}
