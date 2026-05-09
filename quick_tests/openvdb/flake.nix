{
  description = "OpenVDB Hello World Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {

        nativeBuildInputs = with pkgs; [
          cmake
          gnumake
          gcc
        ];

        # buildInputs contains the libraries your code links against
        buildInputs = with pkgs; [
          openvdb
          onetbb
          boost
          c-blosc
          zlib
          jemalloc
        ];

        shellHook = ''
            export OpenVDB_DIR="${pkgs.openvdb.dev}/lib/cmake/OpenVDB/"
        '';
      };
    };
}
