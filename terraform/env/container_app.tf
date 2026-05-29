resource "azurerm_container_app_environment" "lerpz" {
  name                = local.container_env_name
  resource_group_name = azurerm_resource_group.lerpz.name
  location            = azurerm_resource_group.lerpz.location

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
  workload_profile_name        = "Consumption"

  identity {
    type         = "UserAssigned"
    identity_ids = [azurerm_user_assigned_identity.runtime.id]
  }

  registry {
    server   = local.shared.acr_login_server
    identity = azurerm_user_assigned_identity.runtime.id
  }

  ingress {
    external_enabled = true
    target_port      = var.container_port

    traffic_weight {
      latest_revision = true
      percentage      = 100
    }
  }

  lifecycle {
    ignore_changes = [
      template[0].container[0].image,
    ]
  }

  template {
    min_replicas = 0
    max_replicas = 1

    container {
      name   = local.container_app_name
      image  = var.container_image
      cpu    = 0.25
      memory = "0.5Gi"

      env {
        name  = "PUBLIC_API_URL"
        value = var.public_api_url
      }

      env {
        name  = "PUBLIC_ENTRA_ID_TENANT_ID"
        value = var.public_entra_id_tenant_id
      }

      env {
        name  = "PUBLIC_ENTRA_ID_CLIENT_ID"
        value = var.public_entra_id_client_id
      }

      env {
        name  = "PUBLIC_ENTRA_ID_SCOPE"
        value = var.public_entra_id_scope
      }

      env {
        name  = "PUBLIC_ENTRA_ID_REDIRECT_URI"
        value = var.public_entra_id_redirect_uri
      }

      env {
        name  = "PUBLIC_ENTRA_ID_LOGOUT_URI"
        value = var.public_entra_id_logout_uri
      }
    }
  }
}

# Custom domain — lerpz.com / stag.lerpz.com
#
# IMPORTANT — three-phase setup (Azure requires the hostname to be registered
# on the container app *before* a managed certificate can be issued):
#
#  Phase 1 — DNS records:
#    After the very first `terraform apply`, retrieve the static IP and domain
#    verification ID from the outputs, then create the following DNS records at
#    your registrar:
#
#      terraform output container_app_environment_static_ip
#      terraform output -raw container_app_environment_domain_verification_id
#
#      <domain>        A    <static_ip>
#      asuid.<domain>  TXT  <domain_verification_id>
#
#    Wait for DNS to propagate before proceeding:
#
#      dig A <domain> +short
#      dig TXT asuid.<domain> +short
#
#  Phase 2 — register the hostname (no cert yet):
#    Once DNS has propagated, `azurerm_container_app_custom_domain` below
#    registers the domain on the container app with
#    certificate_binding_type = "Disabled". Run `terraform apply`.
#
#  Phase 3 — issue the managed certificate:
#    Uncomment `azurerm_container_app_environment_managed_certificate` below
#    and run `terraform apply`. Azure will validate ownership via HTTP-01 and
#    issue a free managed TLS certificate.
#    Once the certificate is provisioned, update `azurerm_container_app_custom_domain`
#    to set certificate_binding_type = "SniEnabled" and
#    container_app_environment_certificate_id to the certificate resource's id,
#    then run `terraform apply` one final time.

resource "azurerm_container_app_custom_domain" "lerpz_com" {
  name                     = local.domain
  container_app_id         = azurerm_container_app.lerpz_website.id
  certificate_binding_type = "SniEnabled"
}

# Phase 3 — uncomment once the hostname is registered (Phase 2 applied)
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
  scope                = local.shared.acr_id
  role_definition_name = "AcrPull"
  principal_id         = azurerm_user_assigned_identity.runtime.principal_id
}
