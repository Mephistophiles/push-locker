{ lib, rustPlatform, pkgs, pkg-config, protobuf }:

rustPlatform.buildRustPackage rec {
  pname = "push-locker";
  version = "0.1.0";

  src = pkgs.fetchFromGitHub {
    owner = "Mephistophiles";
    repo = pname;
    rev = "aca6e8ceadfdce3f513a7ba10ae2c0a9178049f1";
    sha256 = "1d5aw0dmv7gkdcliq1mwz7n43kbdsyv3h6iv7zrrisn64f0269wh";
  };

  cargoSha256 = "0avz1x4kyszzq7rhg8gn7nh6qldss5vjfr59263q8rfx03djh79s";

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
