{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = {
      check = let
        checks = [
          "nixfmt-check"
          "hadolint-check"
          "deadnix-check"
          "spell-check"
          "docs-static"
        ];
        toCommand = check: ''
                  echo "🧐Checking ${check}..."
          				nix build .\#${check} --no-warn-dirty
                  if [ $? -eq 1 ]; then 
                    echo "❌Check ${check} FAILED"
                  else 
                    printf "\033[1A" # Remove the Checking... line                   
                    echo -e "\r\e[K✅Check ${check} PASSED"
                  fi
          			'';
        script = pkgs.lib.concatMapStrings toCommand checks;
      in pkgs.writeShellScriptBin "check" script;
    };
  };
}
