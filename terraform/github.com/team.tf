
data "github_user" "docs" {
  username = "JafarAz"
}

resource "github_repository_collaborators" "roles" {
  repository = "composable"

  user {
    permission = "write"
    username   = data.github_user.docs.name
  }
}