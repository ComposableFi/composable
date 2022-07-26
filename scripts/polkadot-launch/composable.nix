# definition of parachain
# TODO: replace with zombienet
# because it allows to specify more things and tests
# more structured and portable and officially endrosed by parity
# NOTE: so with nix it is easier to build own (nix+curl+websockat)
# NOTE: also nix allows build multi parachains, unlike composable
# NOTE: produce json/js code so instead of copy paste, people will just reference host spect
builtins.fromJSON (builtins.readFile ./composable.json)