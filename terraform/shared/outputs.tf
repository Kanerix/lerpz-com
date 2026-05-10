output "acr_id" {
  value       = azurerm_container_registry.lerpz.id
  description = "Resource ID of the shared Azure Container Registry."
}

output "acr_login_server" {
  value       = azurerm_container_registry.lerpz.login_server
  description = "Login server URL for the Azure Container Registry."
}

output "deployer_id" {
  value       = azurerm_user_assigned_identity.deployer.id
  description = "Resource ID of the deployer managed identity."
}

output "deployer_client_id" {
  value       = azurerm_user_assigned_identity.deployer.client_id
  description = "Client ID of the deployer managed identity."
}

output "deployer_principal_id" {
  value       = azurerm_user_assigned_identity.deployer.principal_id
  description = "Principal ID of the deployer managed identity."
}
