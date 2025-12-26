output "bucket_name" {
  value       = module.gcp_app.bucket_name
  description = "Provisioned storage bucket name"
}

output "bucket_url" {
  value       = module.gcp_app.bucket_url
  description = "Provisioned storage bucket URL"
}
