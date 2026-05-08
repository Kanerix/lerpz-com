# External Resource Group
#
# Holds resources that exist outside the application lifecycle: the Terraform
# state backend and the deployment managed identity.
#
# These resources are only *created* in the prod state. The stag state
# references them via data sources.

resource "azurerm_resource_group" "ext" {
  count    = var.environment == "prod" ? 1 : 0
  name     = "lerpz-rg-ext"
  location = local.location
}

data "azurerm_resource_group" "ext" {
  count = var.environment == "prod" ? 0 : 1
  name  = "lerpz-rg-ext"
}

resource "azurerm_storage_account" "tfstate" {
  count                    = var.environment == "prod" ? 1 : 0
  name                     = "lerpztfstate"
  resource_group_name      = azurerm_resource_group.ext[0].name
  location                 = azurerm_resource_group.ext[0].location
  account_tier             = "Standard"
  account_replication_type = "LRS"
  account_kind             = "StorageV2"

  allow_nested_items_to_be_public = false
  min_tls_version                 = "TLS1_2"

  blob_properties {
    versioning_enabled = true
  }
}

resource "azurerm_storage_container" "tfstate" {
  count                 = var.environment == "prod" ? 1 : 0
  name                  = "tfstate"
  storage_account_id    = azurerm_storage_account.tfstate[0].id
  container_access_type = "private"
}

resource "azurerm_user_assigned_identity" "deployer" {
  count               = var.environment == "prod" ? 1 : 0
  name                = "lerpz-deployer"
  resource_group_name = azurerm_resource_group.ext[0].name
  location            = azurerm_resource_group.ext[0].location
}

data "azurerm_user_assigned_identity" "deployer" {
  count               = var.environment == "prod" ? 0 : 1
  name                = "lerpz-deployer"
  resource_group_name = "lerpz-rg-ext"
}

resource "azurerm_role_assignment" "deployer_tfstate" {
  count                = var.environment == "prod" ? 1 : 0
  scope                = azurerm_storage_account.tfstate[0].id
  role_definition_name = "Storage Blob Data Contributor"
  principal_id         = local.deployer.principal_id
}

resource "azurerm_role_assignment" "deployer_acr_push" {
  count                = var.environment == "prod" ? 1 : 0
  scope                = local.acr.id
  role_definition_name = "AcrPush"
  principal_id         = local.deployer.principal_id
}

resource "azurerm_role_assignment" "deployer_contributor" {
  scope                = azurerm_resource_group.lerpz.id
  role_definition_name = "Contributor"
  principal_id         = local.deployer.principal_id
}
