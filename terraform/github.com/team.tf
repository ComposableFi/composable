
data "github_user" "docs" {
  username = "JafarAz"
}

data "github_team" "sre" {
  slug = "sre"
}

data "github_team" "devs" {
  slug = "developers"
}

data "github_team" "product" {
  slug = "product-mgmt"
}

data "github_user" "ops" {
  username = "dzmitry-lahoda"
}

resource "github_repository_collaborators" "roles" {
  repository = "composable"

  team {
    permission = "push"
    team_id    = data.github_team.devs.slug
  }

  user {
    permission = "admin"
    username   = data.github_user.ops.name
  }

  user {
    permission = "maintain"
    username   = "JafarAz"
  }

  user {
    permission = "admin"
    username   = "kkast"
  }

  user {
    permission = "maintain"
    username   = "RustNinja"
  }

  team {
    permission = "maintain"
    team_id    = data.github_team.product.slug
  }

  team {
    permission = "maintain"
    team_id    = data.github_team.sre.slug
  }
}
