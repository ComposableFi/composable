
locals {
  labels = {
    "misc"             = "I marked PR by `misc` label if it should not be in release notes"
    "dependencies"     = "bot"
    "lfs-detected!"    = "bot: Warning Label for use when LFS is detected in the commits of a Pull Request"
    "needs-benchmarks" = "bot: Runs benchmarks on target hardware"
  }
}

resource "github_issue_label" "label" {
  for_each    = local.labels
  repository  = "composable"
  name        = each.key
  description = each.value
  color       = substr(sha1(each.key), 0, 6)
}
