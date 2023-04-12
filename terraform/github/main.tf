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
