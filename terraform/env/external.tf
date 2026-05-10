# Deployer Contributor Role
#
# Grants the shared deployer managed identity the Contributor role on this
# environment's resource group so GitHub Actions can deploy here.
resource "azurerm_role_assignment" "deployer_contributor" {
  scope                = azurerm_resource_group.lerpz.id
  role_definition_name = "Contributor"
  principal_id         = local.shared.deployer_principal_id
}
