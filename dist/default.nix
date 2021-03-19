{ lib, rustPlatform, pkgs, pkg-config, protobuf }:

rustPlatform.buildRustPackage rec {
  pname = "push-locker";
  version = "0.1.3";

  src = pkgs.fetchFromGitHub {
    owner = "Mephistophiles";
    repo = pname;
    rev = "v${version}";
    sha256 = "09bdjmm2rv0bivgl98f36625b8sa4sjmi4ll066vvlqw7wz9s701";
  };

  cargoSha256 = "0hdsvgjmdfqb0cwn3flv9qwirx88k0cqz1fh2nvwmx5zvj83fnz7";

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
