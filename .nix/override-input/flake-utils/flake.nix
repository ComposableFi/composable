{
  description = "Our overrides for ci on linux";
  outputs = { self }: {
    lib = {
      eachDefaultSystem = f : 
        let 
          system = "x86_64-linux";
          ret = f system;                    
        in 
         # maps `packages` 
         # into `packages.${system}.packages`
         builtins.foldl' (attrs: key: { ${key} = { ${system} = ret.${key}; };}) {} (builtins.attrNames ret);
    };
  };
}
