resource "github_repository" "composable-nix" {
  name        = "composable.nix"
  description = "Aggregator of foundry.nix, cosmos.nix, ethereum.nix, and all other nix tooling"

  visibility = "public"
}

resource "github_repository_collaborators" "roles" {
  repository = github_repository.composable-nix.name
  user {
    permission = "admin"
    username   = "dzmitry-lahoda"
  }
}

