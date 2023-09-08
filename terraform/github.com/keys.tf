resource "tls_private_key" "CI_SSH_KEY" {
  algorithm = "RSA"
  rsa_bits  = 4096
}

resource "github_actions_environment_secret" "CI_SSH_KEY" {
    environment = tls_private_key.CI_SSH_KEY.public_key_openssh
    repository = data.github_repository.composable
    secret_name = "CI_SSH_KEY"   
}