{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    # for file fetching
    packages = [
      (pkgs.python3.withPackages (python-pkgs: [
        python-pkgs.requests
      ]))
    ];

    RUST_BACKTRACE = 1;
  }
