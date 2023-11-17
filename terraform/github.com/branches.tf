resource "github_branch_protection" "main" {
  # Branch name pattern
  pattern          = "main"
  
  repository_id    = "composable"
  
  # Do not allow bypassing the above settings
  enforce_admins   = true
  # Allow deletions
  allows_deletions = false
  # Require status checks to pass before merging
  required_status_checks {
    # Require branches to be up to date before merging
    strict   = false
    # Status checks that are required.
    contexts = ["pr-workflow-check / draft-release-check"]
  }

  # Require a pull request before merging
  required_pull_request_reviews {
    
    # Require approvals
    required_approving_review_count = 0
    
    # Dismiss stale pull request approvals when new commits are pushed
    dismiss_stale_reviews           = true

    # Require review from Code Owners
    require_code_owner_reviews      = false
    
    # Allow specified actors to bypass required pull requests
    pull_request_bypassers          = []
    
    # Restrict who can dismiss pull request reviews
    restrict_dismissals             = false
    dismissal_restrictions = [
    ]

    # Require approval of the most recent reviewable push
    require_last_push_approval = true
  }


  # Allow force pushes
  allows_force_pushes             = false
  # Require conversation resolution before merging
  require_conversation_resolution = true
  # Require signed commits
  require_signed_commits          = true
  # Require linear history
  required_linear_history         = false
  
  # Require merge queue
  # https://github.com/integrations/terraform-provider-github/issues/1481
  # Merge method = Squash and Merge
  # Only merge non-failing pull requests = true

  #  Restrict who can push to matching branches
  push_restrictions = [
  ]
}
