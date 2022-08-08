# definition of parachain
# TODO: replace with zombienet
# because it allows to specify more things and tests
# more structured and portable and officially endrosed by parity
# so with nix it is easier to build own (nix+curl+websockat)
# also nix allows build multi parachains, unlike composable
# produce json/js code so instead of copy paste, people will just reference host spec
# will be decided when we have more XCMP/XCVM stuff from devops and devs sides done and see obstacles 
builtins.fromJSON (builtins.readFile ./composable.json)
