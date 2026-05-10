resource "azurerm_user_assigned_identity" "deployer" {
  name                = "lerpz-deployer"
  resource_group_name = azurerm_resource_group.ext.name
  location            = azurerm_resource_group.ext.location
}

resource "azurerm_role_assignment" "deployer_tfstate" {
  scope                = azurerm_storage_account.tfstate.id
  role_definition_name = "Storage Blob Data Contributor"
  principal_id         = azurerm_user_assigned_identity.deployer.principal_id
}

resource "azurerm_role_assignment" "deployer_acr_push" {
  scope                = azurerm_container_registry.lerpz.id
  role_definition_name = "AcrPush"
  principal_id         = azurerm_user_assigned_identity.deployer.principal_id
}
