
locals {
  labels = {
    "misc"             = "I marked PR by `misc` label if it should not be in release notes"
    "wip"              = "Work in Progress, #WIP"
    "dependencies"     = "bot"
    "lfs-detected!"    = "bot: Warning Label for use when LFS is detected in the commits of a Pull Request"
    "needs-benchmarks" = "bot: Runs benchmarks on target hardware"
    "stale-item"       = "bot: Stale PRs and issues handling"
    "stale-branch"     = "bot: Stale branches handling"
    "check"            = "run checks for draft PRs"
  }
}

resource "github_issue_label" "label" {
  for_each    = local.labels
  repository  = "composable"
  name        = each.key
  description = "${each.value} #owned:terraform"
  color       = "FFFFFF"
}
