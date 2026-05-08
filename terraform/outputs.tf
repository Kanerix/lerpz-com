output "acr_login_server" {
  value       = local.acr.login_server
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
    Create an A record at your DNS registrar pointing the environment domain to this IP
    before running the second `terraform apply` to provision the managed TLS
    certificate.
  EOT
}

output "container_app_environment_domain_verification_id" {
  value       = azurerm_container_app_environment.lerpz.custom_domain_verification_id
  description = <<-EOT
    Domain ownership verification token for the Container App Environment.
    Create a TXT record at your DNS registrar before binding a custom domain:

      asuid.<domain>  TXT  <this value>

    This is required by Azure to prove you control the hostname before it will
    register it or issue a managed TLS certificate.
  EOT
  sensitive   = true
}

output "container_app_environment_default_domain" {
  value       = azurerm_container_app_environment.lerpz.default_domain
  description = <<-EOT
    Default domain of the Container App Environment — useful as a CNAME target
    for subdomain bindings.
  EOT
}

output "deployer_client_id" {
  value       = local.deployer.client_id
  description = <<-EOT
    Client ID of the deployer managed identity. Set this as AZURE_CLIENT_ID in
    your GitHub Actions workflow.
  EOT
}

output "github_repository" {
  value       = var.environment == "prod" ? github_repository.lerpz_core[0].full_name : null
  description = "Full name of the GitHub repository (org/repo). Only set in prod state."
}
