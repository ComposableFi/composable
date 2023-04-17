
locals {
  labels = {
    # =================== GitHub Issue and PR labels =========================
    "D-fe"       = "I marked PR by `fe` label if it needs frontend deploy to be in production"
    "D-node"     = "I marked PR by `D-node` label if it needs node redeploy to be in production"
    "D-on-chain" = "I marked PR by `D-on-chain` label if it needs validation function or contract upgrade"
    "D-index"    = "I marked PR by `D-index` label if it needs Subsquid redeploy to be in production"
    "D-docs"     = "I marked PR by `D-docs` label if it needs docs publish"
    "D-dao"      = "I marked PR by `D-dao` label if it needs organizational changes applied"

    "misc"          = "I marked PR by `misc` label if it should not be in release notes"
    "dependencies"  = "bot"
    "lfs-detected!" = "Warning Label for use when LFS is detected in the commits of a Pull Request"
    # ==================================================================

  }
}

resource "github_issue_label" "label" {
  for_each    = local.labels
  repository  = "composable"
  name        = each.key
  description = each.value
  color       = substr(sha1(each.key), 0, 6)
}

