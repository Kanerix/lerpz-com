provider "azurerm" {
  features {}
}

provider "github" {
  owner = local.github_org
}

data "azurerm_client_config" "current" {}

data "azurerm_subscription" "current" {}

resource "azurerm_resource_group" "ext" {
  name     = "lerpz-rg-ext"
  location = local.location
}

resource "azurerm_storage_account" "tfstate" {
  name                     = "lerpztfstate"
  resource_group_name      = azurerm_resource_group.ext.name
  location                 = azurerm_resource_group.ext.location
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
  name                  = "tfstate"
  storage_account_id    = azurerm_storage_account.tfstate.id
  container_access_type = "private"
}
