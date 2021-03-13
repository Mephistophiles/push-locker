{ lib, rustPlatform, pkgs, pkg-config, protobuf }:

rustPlatform.buildRustPackage rec {
  pname = "push-locker";
  version = "0.1.0";

  src = pkgs.fetchFromGitHub {
    owner = "Mephistophiles";
    repo = pname;
    rev = "b98001864eb6c7aaeea55851315e1fb7937a9b6b";
    sha256 = "01dji38cvrz878cd05zyxvy3xysf4scg2axq3lyp04bf66qc7cj7";
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
