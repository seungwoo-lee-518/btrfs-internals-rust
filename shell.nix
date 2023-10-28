{
  pkgs ? import <nixpkgs> {}
}:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      btrfs-progs
      cargo
      rustc
      rust-analyzer
    ];
  }
