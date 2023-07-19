variable "GITHUB_TOKEN" {
  type      = string
  sensitive = true
}

variable "RELEASE_GITHUB_TOKEN" {
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
      version = "5.23.0"
    }
  }
}

provider "github" {
  owner = "ComposableFi"
  token = var.GITHUB_TOKEN
}

terraform {
  backend "remote" {
    hostname     = "app.terraform.io"
    organization = "ComposableFi"

    workspaces {
      name = "composable"
    }
  }
}

resource "github_actions_secret" "RELEASE_GITHUB_TOKEN" {
  repository      = "composable"
  secret_name     = "RELEASE_GITHUB_TOKEN"
  plaintext_value = var.RELEASE_GITHUB_TOKEN
}