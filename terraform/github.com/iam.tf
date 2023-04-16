
data "github_user" "fe-lead" {
  username = "fl-y"
}

data "github_user" "parachain-lead" {
  username = "fl-y"
}

data "github_user" "bot" {
  username = "g-la-d-os"
}

data "github_app" "mergify" {
  slug = "mergify"
}
