# Azure Container Registry (ACR)
#
# ACR is shared across environments — it is only *created* in the prod state
# and referenced via a data source in the stag state.
# The name (var.acr_name) must be globally unique across all of Azure.

resource "azurerm_container_registry" "lerpz" {
  count               = var.environment == "prod" ? 1 : 0
  name                = var.acr_name
  resource_group_name = azurerm_resource_group.lerpz.name
  location            = azurerm_resource_group.lerpz.location
  sku                 = "Basic"
  admin_enabled       = false
}

data "azurerm_container_registry" "lerpz" {
  count               = var.environment == "prod" ? 0 : 1
  name                = var.acr_name
  resource_group_name = "lerpz-rg-ext"
}
