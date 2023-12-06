{ pkgs ? import <nixpkgs> { system = builtins.currentSystem; }
, lib ? pkgs.lib
}:

let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    src = ./.;

    cargoLock.lockFile = ./Cargo.lock;

    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = [ pkgs.libudev-zero ];

    postInstall = ''
      mkdir -p $out/etc/udev/rules.d
      cp os_support/10-miractl.rules $out/etc/udev/rules.d/10-miractl.rules
    '';
  }
