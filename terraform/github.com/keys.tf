resource "tls_private_key" "CI_SSH_KEY" {
  algorithm = "ECDSA"
}

resource "github_actions_environment_secret" "CI_SSH_KEY" {
    environment = tls_private_key.CI_SSH_KEY.public_key_openssh
    repository = data.github_repository.composable.name
    secret_name = "CI_SSH_KEY"   
}