{
  pkgs ? import <nixpkgs> { },
}:

# Ensure libpq is available at link time (fixes: ld: cannot find -lpq)
with pkgs;
mkShell {
  packages = [
    pkg-config
    openssl
    postgresql
  ];

  # Help build scripts/linker find headers and libs in Nix store
  shellHook = ''
    export PKG_CONFIG_PATH="${postgresql.dev}/lib/pkgconfig:${openssl.dev}/lib/pkgconfig''${PKG_CONFIG_PATH:+:}''${PKG_CONFIG_PATH:-}"
    export LIBRARY_PATH="${postgresql.lib}/lib:${openssl.out}/lib''${LIBRARY_PATH:+:}''${LIBRARY_PATH:-}"
  '';
}
