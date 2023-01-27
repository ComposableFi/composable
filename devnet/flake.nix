{
  description = "Make nixops top level attribute";
  inputs = { composable = { url = "path:../"; }; };
  outputs = { self, composable }: {
    nixopsConfigurations.default =
      composable.nixopsConfigurations.x86_64-linux.default;
  };
}
