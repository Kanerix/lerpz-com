# Runtime Managed Identity
#
# A dedicated User-Assigned identity for the container app at runtime.
# Kept separate from the deployer identity so runtime and deployment
# concerns have distinct, minimal permission sets.

resource "azurerm_user_assigned_identity" "runtime" {
  name                = "lerpz-runtime"
  resource_group_name = azurerm_resource_group.lerpz.name
  location            = azurerm_resource_group.lerpz.location
}

resource "azurerm_role_assignment" "acr_pull" {
  scope                = azurerm_container_registry.lerpz.id
  role_definition_name = "AcrPull"
  principal_id         = azurerm_user_assigned_identity.runtime.principal_id
}

# Container App Environment

resource "azurerm_container_app_environment" "lerpz" {
  name                               = "lerpz-env"
  resource_group_name                = azurerm_resource_group.lerpz.name
  location                           = azurerm_resource_group.lerpz.location
  infrastructure_resource_group_name = "lerpz-rg-infra"

  workload_profile {
    name                  = "Consumption"
    workload_profile_type = "Consumption"
    minimum_count         = 0
    maximum_count         = 0
  }
}

# Container App

resource "azurerm_container_app" "lerpz_website" {
  name                         = "lerpz-website"
  resource_group_name          = azurerm_resource_group.lerpz.name
  container_app_environment_id = azurerm_container_app_environment.lerpz.id
  revision_mode                = "Single"

  identity {
    type         = "UserAssigned"
    identity_ids = [azurerm_user_assigned_identity.runtime.id]
  }

  registry {
    server   = azurerm_container_registry.lerpz.login_server
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
      name   = "lerpz-website"
      image  = var.container_image
      cpu    = 0.25
      memory = "0.5Gi"
    }
  }

  # The role assignment and runtime identity are created before the container
  # app (no dependency cycle), so no depends_on is needed.
}

# Custom domain — lerpz.com
#
# IMPORTANT — two-phase DNS setup:
#
#  Phase 1 (before managed certificate):
#    After the first `terraform apply`, retrieve the environment's static IP
#    from the `container_app_environment_static_ip` output and create an
#    A record at your registrar:
#
#      lerpz.com  →  A  →  <static_ip>
#
#    Also add the TXT verification record shown in the Azure Portal / CLI for
#    the custom domain (asuid.<hostname> TXT record) if required by Azure.
#
#  Phase 2 (certificate provisioning):
#    Once the DNS A record has propagated, run `terraform apply` again.
#    Azure will perform HTTP-01 validation and issue a free managed TLS cert.
#    The `azurerm_container_app_custom_domain` resource will then bind it.

resource "azurerm_container_app_environment_managed_certificate" "lerpz_com" {
  name                         = "lerpz-com-cert"
  container_app_environment_id = azurerm_container_app_environment.lerpz.id
  subject_name                 = "lerpz.com"
  domain_control_validation    = "HTTP"
}

resource "azurerm_container_app_custom_domain" "lerpz_com" {
  name                     = "lerpz.com"
  container_app_id         = azurerm_container_app.lerpz_website.id
  certificate_binding_type = "SniEnabled"

  # The managed certificate ID is computed by Azure once it validates the
  # domain — we cannot set it directly. We depend on the cert resource so
  # Terraform waits for it to be provisioned before creating this binding.
  depends_on = [azurerm_container_app_environment_managed_certificate.lerpz_com]
}
