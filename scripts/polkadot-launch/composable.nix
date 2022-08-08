# definition of parachain
# TODO: replace with zombienet
# because it allows to specify more things and tests
# more structured and portable and officially endrosed by parity
# NOTE: so with nix it is easier to build own (nix+curl+websockat)
# TODO: use network-builder.nix after full nix migraiton
builtins.fromJSON (builtins.readFile ./composable.json)
