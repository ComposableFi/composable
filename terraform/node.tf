resource "google_compute_instance" "node" {
  name         = "nixos44s"
  machine_type = "n2-standard-2"

  project = var.project
  depends_on = [
    time_sleep.google_service_account-default
  ]
  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-11"
    }
  }
  metadata = {
    enable-oslogin = true
    //ssh-keys = "dzmitry:ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQDlfhqCSy/AzSWR4CYmbf7Vu0HyXPaoz2/1WB7XJ6eRsakS7p++lfvUejnUo9ZRdMdl4HR5waF2HXbHPMcjXrHljV58uo6NKTLeunVFqdQxfUuNIGw9cjdvKJCw1ig+QGTxEdEdyQY8WzNG8nObmRl0ratsf+IEzEJnkQe2WsD9itbcHf0g5XACTAVbx2CJua1QUbReYsxcZGHw7a+nkLyS3ErXc4xkVl+265PRGe48xCpd/wxFK9TbJhniBr6KQ7DatH1ibLQymBJLxFUuPmiILWR4nDyeoBuvlXKA4uSlT0CAUq+AaeC+lxvJbu04XbCrRHkzib5nBduTgSvBvPsfxNhat3/n1PamDHggybnR0oy5yyD/2H5Kpw5R3q8Vn4QUDW63ObPP8emvRlyAKusIeH3dK/tvZ0guLxENSqnC6yNnfep1RhaRUw4tcHzCYIflBQEzaKijH8hZXRSG0yU7Mq8/yNFchHX+PFPFWyzKiKgIlwRETjEvycqloC3+yqc= dzmitry@composable.finance"
  }

  scratch_disk {
    interface = "SCSI"
  }

  network_interface {
    network = "default"

    access_config {
      // Ephemeral public IP
    }
  }

  metadata_startup_script = "curl -L https://github.com/nix-community/nixos-images/releases/download/nixos-unstable/nixos-kexec-installer-x86_64-linux.tar.gz | tar -xzf- -C /root && /root/kexec/run"

  service_account {
    email  = google_service_account.default.email
    scopes = ["cloud-platform"]
  }
}
