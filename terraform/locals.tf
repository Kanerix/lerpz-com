locals {
  location     = "West Europe"
  github_org   = "kanerix"
  environments = ["prod", "stag"]

  # Per-environment derived names
  resource_group_name   = "lerpz-${var.environment}-rg"
  container_app_name    = "lerpz-website-${var.environment}"
  container_env_name    = "lerpz-env-${var.environment}"
  runtime_identity_name = "lerpz-runtime-${var.environment}"
  domain                = var.environment == "prod" ? "lerpz.com" : "stag.lerpz.com"

  # Shared resource references — resolved from either the managed resource
  # (prod state) or a data source (stag state) via the one(concat(...)) pattern.
  acr         = one(concat(azurerm_container_registry.lerpz[*], data.azurerm_container_registry.lerpz[*]))
  deployer    = one(concat(azurerm_user_assigned_identity.deployer[*], data.azurerm_user_assigned_identity.deployer[*]))
  ext_rg_name = one(concat(azurerm_resource_group.ext[*], data.azurerm_resource_group.ext[*])).name
}
