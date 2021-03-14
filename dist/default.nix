{ lib, rustPlatform, pkgs, pkg-config, protobuf }:

rustPlatform.buildRustPackage rec {
  pname = "push-locker";
  version = "0.1.0";

  src = pkgs.fetchFromGitHub {
    owner = "Mephistophiles";
    repo = pname;
    rev = "333f1444b0820990d4b3c94b18f8801e61222098";
    sha256 = "0fbhymcxhz76lifryggrjh4hphq86gav4w5c0bxcrsvdrbkl33rl";
  };

  cargoSha256 = "0rck7qkp9pf3nr8kglf48dczamd2gn3bmgx4bggl7rfxpqhixbjc";

  nativeBuildInputs = [ pkg-config pkgs.python3 pkgs.protobuf pkgs.rustfmt ];
  buildInputs = [];

  postInstall = ''
    install -D -m755 "dist/pre-push" "$out/share/pushlock/pre-push"
  '';

  meta = with lib; {
    description = "A utility for merge window reservation. ";
    homepage = "https://github.com/Mephistophiles/push-locker";
    license = licenses.gpl3Plus;
    maintainers = [];
  };
}
