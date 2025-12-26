resource "google_storage_bucket" "app_bucket" {
  name          = var.bucket_name
  location      = var.region
  force_destroy = true
}
