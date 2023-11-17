variable "GITHUB_TOKEN" {
  type      = string
  sensitive = true
}

variable "RELEASE_GITHUB_TOKEN" {
  type      = string
  sensitive = true
}

variable "CI_COSMOS_MNEMONIC" {
  type      = string
  sensitive = true
}

variable "CACHIX_AUTH_TOKEN" {
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
      version = "5.42.0"
    }
  }
}

provider "github" {
  owner = "ComposableFi"
  token = var.GITHUB_TOKEN
}

terraform {
  backend "remote" {
    hostname = "app.terraform.io"
    organization = "ComposableFi"

    workspaces {
      name = "composable"
    }
  }
}

resource "github_actions_secret" "RELEASE_GITHUB_TOKEN" {
  repository       = "composable"
  secret_name      = "RELEASE_GITHUB_TOKEN"
  plaintext_value  = var.RELEASE_GITHUB_TOKEN
}

resource "github_actions_secret" "CI_COSMOS_MNEMONIC" {
  repository       = "composable"
  secret_name      = "CI_COSMOS_MNEMONIC"  
  plaintext_value  = var.CI_COSMOS_MNEMONIC
}

resource "github_actions_secret" "CACHIX_AUTH_TOKEN" {
  repository       = "composable"
  secret_name      = "CACHIX_AUTH_TOKEN"
  plaintext_value  = var.CACHIX_AUTH_TOKEN
}

resource "github_actions_secret" "cvm_CACHIX_AUTH_TOKEN" {
  repository       = "cvm"
  secret_name      = "CACHIX_AUTH_TOKEN"
  plaintext_value  = var.CACHIX_AUTH_TOKEN
}