variable "GITHUB_TOKEN" {
  type      = string
  sensitive = true
}


data "github_repository" "self" {
  full_name = "ComposibleFi/composable"
}

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
  token = var.GITHUB_TOKEN
}