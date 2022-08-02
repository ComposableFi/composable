{pkgs, crane-stable}:
crane-stable.buildPackage {
              src = pkgs.fetchFromGitHub {
                owner = "rust-lang";
                repo = "mdBook";
                rev = "40c06f5e774924bef97d339cf8c64343c9056d86";
                hash = "sha256-ggcyOsA4cyo5l87cZmOMI0w1gCzmWy9NRJiWxjBdB1E=";
              };          
            }