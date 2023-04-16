resource "github_repository_tag_protection" "release" {
    repository      = "composable"
    pattern         = "release-v[0-9]+\.[0-9]+\.[0-9]+"
}