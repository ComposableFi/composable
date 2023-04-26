
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
  slug = "@ComposableFi/sre" 
}

data "github_team" "devs" {
  slug = "@ComposableFi/developers" 
}

data "github_team" "product" {
  slug = "@ComposableFi/product-mgmt" 
}

# resource "github_repository_collaborators" "roles" {
#   repository = "composable"

#   user {
#     permission = "write"
#     username   = data.github_user.docs.name
#   }
#   user {
#     permission = "admin"
#     username   = data.github_user.ops.name
#   }
# }