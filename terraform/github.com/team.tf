
data "github_user" "docs" {
  username = "JafarAz"
}


data "github_user" "ops" {
  username = "dzmitry-lahoda"
}

data "github_user" "bot" {
  username = "g-la-d-os"
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


resource "github_repository_collaborators" "roles" {
  repository = "composable"

  team {
    permission = "maintain"
    team_id    = data.github_team.devs.slug
  }

  user {
    permission = "admin"
    username   = data.github_user.ops.name
  }

  user {
    permission = "push"
    username   = "JafarAz"
  }

  user {
    permission = "admin"
    username   = "g-la-d-os"
  }

  team {
    permission = "admin"
    team_id    = data.github_team.product.slug
  }

  team {
    permission = "admin"
    team_id    = data.github_team.sre.slug
  }
}
