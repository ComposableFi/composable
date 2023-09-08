
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
    permission = "push"
    username   = "JafarAz"
  }

  user {
    permission = "push"
    username   = "NOOB6942069"
  }

  user {
    permission = "maintain"
    username   = "kkast"
  }

  user {
    permission = "admin"
    username   = "itsbobbyzz"
  }

  user {
    permission = "maintain"
    username   = "RustNinja"
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
