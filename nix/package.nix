{ lib, rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "todo";
  version = "0.1.0";

  src = ../.;

  cargoLock = { lockFile = ../Cargo.lock; };

  meta = {
    description = "a poorly made todolist";
    license = lib.licenses.gpl3;
  };
}
