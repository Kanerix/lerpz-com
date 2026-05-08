resource "azurerm_container_app_environment" "lerpz" {
  name                               = local.container_env_name
  resource_group_name                = azurerm_resource_group.lerpz.name
  location                           = azurerm_resource_group.lerpz.location
  infrastructure_resource_group_name = "lerpz-${var.environment}-rg-infra"

  workload_profile {
    name                  = "Consumption"
    workload_profile_type = "Consumption"
    minimum_count         = 0
    maximum_count         = 0
  }
}

resource "azurerm_container_app" "lerpz_website" {
  name                         = local.container_app_name
  resource_group_name          = azurerm_resource_group.lerpz.name
  container_app_environment_id = azurerm_container_app_environment.lerpz.id
  revision_mode                = "Single"

  identity {
    type         = "UserAssigned"
    identity_ids = [azurerm_user_assigned_identity.runtime.id]
  }

  registry {
    server   = local.acr.login_server
    identity = azurerm_user_assigned_identity.runtime.id
  }

  ingress {
    external_enabled = true
    target_port      = 80

    traffic_weight {
      latest_revision = true
      percentage      = 100
    }
  }

  template {
    min_replicas = 0
    max_replicas = 1

    container {
      name   = local.container_app_name
      image  = var.container_image
      cpu    = 0.25
      memory = "0.5Gi"
    }
  }
}

# Custom domain — lerpz.com
#
# IMPORTANT — three-phase setup (Azure requires the hostname to be registered
# on the container app *before* a managed certificate can be issued):
#
#  Phase 1 — DNS records:
#    After the very first `terraform apply`, retrieve the environment's static
#    IP from the `container_app_environment_static_ip` output and create:
#
#      lerpz.com        A      <static_ip>
#      asuid.lerpz.com  TXT    <domain_verification_id output>
#
#  Phase 2 — register the hostname (no cert yet):
#    Once DNS has propagated, the `azurerm_container_app_custom_domain`
#    resource below registers `lerpz.com` on the container app with
#    certificate_binding_type = "Disabled".  Run `terraform apply`.
#
#  Phase 3 — issue the managed certificate:
#    With the hostname registered, Azure can now validate ownership via
#    HTTP-01 and issue a free managed TLS certificate.  Update
#    certificate_binding_type to "SniEnabled" and set
#    container_app_environment_certificate_id, then run `terraform apply`
#    again.

# Step 1: register the hostname on the container app.
# Start with certificate_binding_type = "Disabled" (Phase 2).
# Once the managed certificate is provisioned, change to "SniEnabled" and
# set container_app_environment_certificate_id (Phase 3).
resource "azurerm_container_app_custom_domain" "lerpz_com" {
  name                     = local.domain
  container_app_id         = azurerm_container_app.lerpz_website.id
  certificate_binding_type = "Disabled"
}

# Step 2: request the managed certificate.
# This can only succeed after the hostname is registered above.
resource "azurerm_container_app_environment_managed_certificate" "lerpz_com" {
  name                         = replace(local.domain, ".", "-")
  container_app_environment_id = azurerm_container_app_environment.lerpz.id
  subject_name                 = local.domain
  domain_control_validation    = "HTTP"

  depends_on = [azurerm_container_app_custom_domain.lerpz_com]
}

resource "azurerm_user_assigned_identity" "runtime" {
  name                = local.runtime_identity_name
  resource_group_name = azurerm_resource_group.lerpz.name
  location            = azurerm_resource_group.lerpz.location
}

resource "azurerm_role_assignment" "acr_pull" {
  scope                = local.acr.id
  role_definition_name = "AcrPull"
  principal_id         = azurerm_user_assigned_identity.runtime.principal_id
}
