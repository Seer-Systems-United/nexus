let
  # Pin to a specific nixpkgs commit for reproducibility.
  pkgs =
    import
      (fetchTarball "https://github.com/NixOS/nixpkgs/archive/24bb1b20a9a57175965c0a9fb9533e00e370c88b.tar.gz")
      { config.allowUnfree = true; };
  py = pkgs.python311Packages.overrideScope (
    final: prev:
    let
      addCudaSolver =
        pkg:
        pkg.overridePythonAttrs (old: {
          buildInputs = (old.buildInputs or [ ]) ++ [ pkgs.cudaPackages.libcusolver.lib ];
        });
      patchedTorch = addCudaSolver prev.torch-bin;
    in
    {
      # Some packages depend on `torch`, while this shell explicitly wants the
      # prebuilt PyTorch wheel. Keep both names pointed at the same derivation.
      torch = patchedTorch;
      torch-bin = patchedTorch;
      torchvision = final.torchvision-bin;
      torchaudio = final.torchaudio-bin;
    }
  );
  hasNvidia =
    builtins.pathExists "/proc/driver/nvidia/version" || builtins.pathExists "/dev/nvidiactl";
  torchPkg = py.torch;
in
pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.python311
    torchPkg

    pkgs.git
    pkgs.ffmpeg
    pkgs.cmake
    pkgs.ninja
    pkgs.gcc
    pkgs.linuxPackages.perf
    pkgs.openssl
    pkgs.postgresql
    pkgs.diesel-cli

    # Vulkan runtime tools/libs for AMD and non-CUDA fallback paths.
    pkgs.vulkan-loader
    pkgs.vulkan-tools
    pkgs.vulkan-validation-layers
    pkgs.libglvnd
    pkgs.mesa

    pkgs.pkg-config
  ]
  ++ pkgs.lib.optionals hasNvidia [
    pkgs.cudatoolkit
  ];

  shellHook = ''
    echo "You are now using a NIX environment"
    export GDK_BACKEND=wayland,x11
    export QT_QPA_PLATFORM="wayland;xcb"
    export VK_LAYER_PATH=${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d
    export HF_HOME="$PWD/.cache/huggingface"
    export TRANSFORMERS_CACHE="$HF_HOME/transformers"
    export RUSTBERT_CACHE="$PWD/.cache/rustbert"
    export LIBTORCH_USE_PYTORCH=1
    export OPENSSL_LIB_DIR=${pkgs.openssl.out}/lib
    export OPENSSL_INCLUDE_DIR=${pkgs.openssl.dev}/include
    export PQ_LIB_DIR=${pkgs.postgresql}/lib
    export PG_CONFIG=${pkgs.postgresql}/bin/pg_config
    export NIX_LDFLAGS="$(printf '%s' "''${NIX_LDFLAGS:-}" | sed -E 's#-rpath /nix/store/[^ ]+-nix-shell/lib[ ]*##g')"
    export PGHOST="''${PGHOST:-127.0.0.1}"
    export PGPORT="''${PGPORT:-55432}"
    export PGUSER="''${PGUSER:-nexus}"
    export PGDATABASE="''${PGDATABASE:-postgres}"
    export DATABASE_URL="''${DATABASE_URL:-postgres://nexus@127.0.0.1:55432/nexus_test}"
    export NEXUS_PG_ROOT="''${NEXUS_PG_ROOT:-$PWD/.postgres-test}"
    export NEXUS_PGDATA="''${NEXUS_PGDATA:-$NEXUS_PG_ROOT/data}"
    export NEXUS_PG_SOCKET_DIR="''${NEXUS_PG_SOCKET_DIR:-$NEXUS_PG_ROOT/run}"
    export NEXUS_PG_LOG="''${NEXUS_PG_LOG:-$NEXUS_PG_ROOT/postgres.log}"

    nexus_pg_start() {
      mkdir -p "$NEXUS_PG_ROOT" "$NEXUS_PG_SOCKET_DIR"

      if [ ! -s "$NEXUS_PGDATA/PG_VERSION" ]; then
        echo "Initializing local PostgreSQL cluster at $NEXUS_PGDATA"
        initdb \
          -D "$NEXUS_PGDATA" \
          --username="$PGUSER" \
          --auth=trust \
          --encoding=UTF8 \
          --locale=C \
          >/dev/null
        {
          echo "listen_addresses = '$PGHOST'"
          echo "port = $PGPORT"
          echo "unix_socket_directories = '$NEXUS_PG_SOCKET_DIR'"
        } >> "$NEXUS_PGDATA/postgresql.conf"
      fi

      if psql -h "$PGHOST" -p "$PGPORT" -U "$PGUSER" -d postgres -c 'select 1' >/dev/null 2>&1; then
        return 0
      fi

      if pg_isready -h "$PGHOST" -p "$PGPORT" >/dev/null 2>&1; then
        echo "PostgreSQL is already listening on $PGHOST:$PGPORT, but $PGUSER cannot connect."
        return 1
      fi

      if [ -s "$NEXUS_PGDATA/postmaster.pid" ] && ! pg_ctl -D "$NEXUS_PGDATA" status >/dev/null 2>&1; then
        rm -f "$NEXUS_PGDATA/postmaster.pid"
      fi

      pg_ctl \
        -D "$NEXUS_PGDATA" \
        -l "$NEXUS_PG_LOG" \
        -o "-k $NEXUS_PG_SOCKET_DIR" \
        start \
        >/dev/null
    }

    nexus_pg_stop() {
      pg_ctl -D "$NEXUS_PGDATA" stop
    }

    nexus_pg_status() {
      pg_ctl -D "$NEXUS_PGDATA" status
    }

    if [ "''${NEXUS_SKIP_POSTGRES_AUTOSTART:-0}" != "1" ]; then
      if nexus_pg_start; then
        echo "PostgreSQL ready on $PGHOST:$PGPORT"
      else
        echo "PostgreSQL failed to start. See $NEXUS_PG_LOG"
      fi
    fi

    if [ -e /proc/driver/nvidia/version ] || [ -e /dev/nvidiactl ] || command -v nvidia-smi >/dev/null 2>&1; then
      export CUDA_PATH=${pkgs.cudatoolkit}
      echo "NVIDIA/CUDA detected. CUDA_PATH=$CUDA_PATH"
    else
      unset CUDA_PATH
      echo "No NVIDIA CUDA device detected. CUDA toolchain not enabled."
    fi
  '';
}
