{ lib, rustPlatform, pkgs, pkg-config, protobuf }:

rustPlatform.buildRustPackage rec {
  pname = "push-locker";
  version = "0.1.1";

  src = pkgs.fetchFromGitHub {
    owner = "Mephistophiles";
    repo = pname;
    rev = "v${version}";
    sha256 = "0r3ajiff0hd8zhdapi8vw559bnys48dr5wj4vgc8y9p56di3ggcb";
  };

  cargoSha256 = "0si8j5j9sxf4y332byi3rs7l0bxl9ix2pjxr5rwh1mxdrl53y18d";

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
