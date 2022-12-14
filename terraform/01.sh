source ./.tfvars
nix profile install nixpkgs#terraform nixpkgs#google-cloud-sdk
terraform init --upgrade
gcloud auth login
gcloud auth application-default login
gcloud projects create $PROJECT
gcloud config set project $PROJECT
gcloud services enable compute.googleapis.com