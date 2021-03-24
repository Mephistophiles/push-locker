{ lib, rustPlatform, pkgs, pkg-config, protobuf }:

rustPlatform.buildRustPackage rec {
  pname = "push-locker";
  version = "0.1.4";

  src = pkgs.fetchFromGitHub {
    owner = "Mephistophiles";
    repo = pname;
    rev = "v${version}";
    sha256 = "047938js7qi1h9n2rxbzd27kvgs7ynqwdrfmip8jnqvfawxi951j";
  };

  cargoSha256 = "047f97mhvj258851rplqg5lz1dhhsx7wmqmgxvds2y20qmhlvhpa";

  nativeBuildInputs = [ pkg-config ];
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
