
data "github_user" "fe-lead" {
  username = "fl-y"
}

data "github_user" "parachain-lead" {
  username = "kkast"
}

data "github_user" "stuff" {
  username = "dzmitry-lahoda"
}

data "github_user" "cto" {
  username = "blasrodri"
}

data "github_user" "sre-lead" {
  username = "bostercf"
}

data "github_user" "sre-bot" {
  username = "ComposableFiRelease"
}

data "github_user" "dev-bot" {
  username = "g-la-d-os"
}

data "github_app" "mergify" {
  slug = "mergify"
}


resource "github_repository_collaborators" "roles" {
  repository = "composable"

  user {
    permission = "maintain"
    username   = data.github_user.parachain-lead.name
  }
}
