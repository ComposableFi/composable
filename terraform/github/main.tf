terraform {
  required_providers {
    github = {
      source  = "integrations/github"
      version = "5.22.0"
    }
  }
}

provider "github" {
  owner = "ComposableFi"
}

data "github_repository" "self" {
  full_name = "ComposibleFi/composable"
}

data "github_user" "fe-lead" {
  username = "fl-y"
}

data "github_user" "bot" {
  username = "g-la-d-os"
}

data "github_app" "mergify" {
  slug = "mergify"
}

resource "github_branch_protection" "main" {
  repository_id    = "composable"
  pattern          = "main"
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
    pull_request_bypassers          = []
    restrict_dismissals             = false

    dismissal_restrictions = [
    ]

    require_last_push_approval = true
  }

  allows_force_pushes             = false
  require_conversation_resolution = true
  require_signed_commits          = true
  required_linear_history         = false

  push_restrictions = [
    data.github_user.bot.node_id,
    data.github_app.mergify.node_id,
  ]
}

resource "github_issue_label" "fe" {
  repository = "composable"
  name       = "fe"
  color      = "#000000"
  description = "I marked PR by `fe` label if it needs frontend deploy to be in production"
}

resource "github_issue_label" "misc" {
  repository = "composable"
  name       = "misc"
  color      = "#000000"
  description = "I marked PR by `misc` label if it should not be in release notes"
}

resource "github_issue_label" "node" {
  repository = "composable"
  name       = "node"
  color      = "#000000"
  description = "I marked PR by `node` label if it needs node redeploy to be in production"
}


resource "github_issue_label" "on-chain" {
  repository = "composable"
  name       = "on-chain"
  color      = "#000000"
  description = "I marked PR by `on-chain` label if it needs runtime upgrade or contract (re)deploy to be in production"
}


resource "github_issue_label" "data" {
  repository = "composable"
  name       = "data"
  color      = "#000000"
  description = "I marked PR by `data` label if it needs Subsquid redeploy to be in production"
}