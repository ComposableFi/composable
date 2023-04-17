resource "github_repository_tag_protection" "release" {
  repository = "composable"
  pattern    = "release*"
}



resource "github_branch_protection" "release" {
  repository_id    = "composable"
  pattern          = "release*"
  enforce_admins   = true
  allows_deletions = false
  required_status_checks {
    strict   = false
    contexts = ["Effect gate, automatically merged if passed"]
  }

  required_pull_request_reviews {
    required_approving_review_count = 1
    dismiss_stale_reviews           = true
    require_code_owner_reviews      = true
    pull_request_bypassers = [
      data.github_user.dev-bot.node_id
    ]
    restrict_dismissals = false

    dismissal_restrictions = [
    ]

    require_last_push_approval = true
  }

  allows_force_pushes = false

  require_conversation_resolution = true
  require_signed_commits          = true
  required_linear_history         = false

  push_restrictions = [
    data.github_user.dev-bot.node_id,
    data.github_user.sre-bot.node_id,
    data.github_user.sre-lead.node_id,
    data.github_user.cto.node_id,
    data.github_user.parachain-lead.node_id,
    data.github_user.stuff.node_id,
  ]
}
