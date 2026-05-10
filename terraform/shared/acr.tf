resource "azurerm_container_registry" "lerpz" {
  name                = "lerpzacr"
  resource_group_name = azurerm_resource_group.ext.name
  location            = azurerm_resource_group.ext.location
  sku                 = "Basic"
  admin_enabled       = false
}
