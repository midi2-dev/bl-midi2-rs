{ pkgs ? import <nixpkgs> { }}:
pkgs.mkShell {
  # Additional tooling
  buildInputs = with pkgs; [
    rust-analyzer # LSP Server
    rustfmt       # Formatter
    clippy        # Linter
    cargo         # Dependency Manager
    rustc         # Compiler

    python313Packages.wheel
    pre-commit
    codespell
  ];

  shellHook = ''
    ./configure-hooks.sh install
  '';
}
