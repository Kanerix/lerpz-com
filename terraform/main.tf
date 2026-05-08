provider "azurerm" {
  features {}
}

provider "github" {
  owner = local.github_org
}

data "azurerm_client_config" "current" {}

data "azurerm_subscription" "current" {}

resource "azurerm_resource_group" "lerpz" {
  name     = local.resource_group_name
  location = local.location
}
