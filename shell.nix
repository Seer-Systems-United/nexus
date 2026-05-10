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
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib ]}:''${LD_LIBRARY_PATH:-}

    if [ -e /proc/driver/nvidia/version ] || [ -e /dev/nvidiactl ] || command -v nvidia-smi >/dev/null 2>&1; then
      export CUDA_PATH=${pkgs.cudatoolkit}
      echo "NVIDIA/CUDA detected. CUDA_PATH=$CUDA_PATH"
    else
      unset CUDA_PATH
      echo "No NVIDIA CUDA device detected. CUDA toolchain not enabled."
    fi
  '';
}
