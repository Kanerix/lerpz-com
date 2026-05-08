# Azure Container Registry (ACR)
#
# ACR names must be globally unique, alphanumeric, 5–50 characters.
# A random 6-character suffix is appended to "lerpz" to satisfy uniqueness.

resource "random_string" "acr_suffix" {
  length  = 6
  upper   = false
  special = false
}

resource "azurerm_container_registry" "lerpz" {
  name                = "lerpz${random_string.acr_suffix.result}"
  resource_group_name = azurerm_resource_group.lerpz.name
  location            = azurerm_resource_group.lerpz.location
  sku                 = "Basic"

  # Admin credentials are disabled; the Container App pulls images via its
  # SystemAssigned managed identity and an AcrPull role assignment instead.
  admin_enabled = false
}
