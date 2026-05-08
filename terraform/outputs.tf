output "acr_login_server" {
  value       = azurerm_container_registry.lerpz.login_server
  description = <<-EOT
    Login server URL for the Azure Container Registry. Use this as
    the registry host when pushing images
    (e.g. docker push <acr_login_server>/myimage:tag).
  EOT
}

output "container_app_url" {
  value       = "https://${azurerm_container_app.lerpz_website.latest_revision_fqdn}"
  description = <<-EOT
    Default FQDN assigned to the Container App by Azure (*.azurecontainerapps.io).
    Useful for smoke-testing before DNS is configured.
  EOT
}

output "container_app_environment_static_ip" {
  value       = azurerm_container_app_environment.lerpz.static_ip_address
  description = <<-EOT
    Public static IP of the Container App Environment.
    Create an A record at your DNS registrar pointing lerpz.com to this IP
    before running the second `terraform apply` to provision the managed TLS
    certificate.
  EOT
}

output "container_app_environment_default_domain" {
  value       = azurerm_container_app_environment.lerpz.default_domain
  description = <<-EOT
    Default domain of the Container App Environment — useful as a CNAME target
    for subdomain bindings.
  EOT
}

output "deployer_client_id" {
  value       = azurerm_user_assigned_identity.deployer.client_id
  description = <<-EOT
    Client ID of the deployer managed identity. Set this as AZURE_CLIENT_ID in
    your GitHub Actions workflow.
  EOT
}

output "github_repository" {
  value       = github_repository.lerpz_core.full_name
  description = "Full name of the GitHub repository (org/repo)."
}
