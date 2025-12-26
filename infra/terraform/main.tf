provider "google" {
  project = var.project_id
  region  = var.region
}

module "gcp_app" {
  source      = "./modules/gcp_app"
  project_id  = var.project_id
  region      = var.region
  bucket_name = var.bucket_name
}
