output "bucket_name" {
  value       = google_storage_bucket.app_bucket.name
  description = "Bucket name"
}

output "bucket_url" {
  value       = google_storage_bucket.app_bucket.url
  description = "Bucket URL"
}
