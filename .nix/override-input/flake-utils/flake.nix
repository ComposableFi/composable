{
  description = "Our overrides for ci on linux";
  outputs = { self }: {
    lib = {
      eachDefaultSystem = f : 
        let 
          system = "x86_64-linux";
          outputs = f system;       
          appendSystem = attrs: key:  
            # https://github.com/numtide/flake-utils/issues/77
            if key == "nixopsConfigurations" then
              { ${key} = outputs.${key}; }
            else
              { ${key} = { ${system} = outputs.${key}; };};           
        in 
         # maps `packages.foobar` 
         # into `packages.${system}.foobar`
         builtins.foldl' appendSystem {} (builtins.attrNames outputs);
    };
  };
}
