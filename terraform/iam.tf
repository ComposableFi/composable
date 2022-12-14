resource "google_service_account" "default" {
  account_id = var.project
  project    = var.project
}

resource "time_sleep" "google_service_account-default" {
  depends_on = [google_service_account.default]

  create_duration = "30s"
}
