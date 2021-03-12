{ pkgs ? import <nixpkgs> {} }:
with pkgs;
pkgs.mkShell {
  buildInputs = [ pkgconfig openssl dbus cmake zlib libgit2 ];
  shellHook = ''export CFG_DISABLE_CROSS_TESTS=1'';
}
