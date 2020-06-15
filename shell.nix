with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "freeipmi-monitoring-build";
  buildInputs = [
    libgcrypt pkgconfig autoconf automake libtool libgcrypt libgpgerror systemd
  ];
}

