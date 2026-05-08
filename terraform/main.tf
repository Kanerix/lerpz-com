terraform {
  required_version = ">= 1.6.0"

  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 4.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.0"
    }
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
  }
}

provider "azurerm" {
  features {}
}

data "azurerm_client_config" "current" {}

data "azurerm_subscription" "current" {}

provider "github" {
  owner = local.github_org
}

# Holds resources related to the application lifecycle.
resource "azurerm_resource_group" "lerpz" {
  name     = "lerpz-rg"
  location = local.location
}

# Holds resources that exist outside the application lifecycle: the Terraform
# state backend and the deployment managed identity.
resource "azurerm_resource_group" "ext" {
  name     = "lerpz-rg-ext"
  location = local.location
}

# Terraform State Storage
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

# Deployment Managed Identity
#
# A User-Assigned Managed Identity for CI/CD pipelines — no long-lived
# credentials, uses federated identity (OIDC) with GitHub Actions.

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

resource "azurerm_role_assignment" "deployer_contributor" {
  scope                = azurerm_resource_group.lerpz.id
  role_definition_name = "Contributor"
  principal_id         = azurerm_user_assigned_identity.deployer.principal_id
}
