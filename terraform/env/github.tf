resource "github_actions_environment_variable" "public_api_url" {
  repository    = "lerpz-com"
  environment   = var.environment
  variable_name = "PUBLIC_API_URL"
  value         = var.public_api_url
}

resource "github_actions_environment_variable" "public_entra_id_tenant_id" {
  repository    = "lerpz-com"
  environment   = var.environment
  variable_name = "PUBLIC_ENTRA_ID_TENANT_ID"
  value         = var.public_entra_id_tenant_id
}

resource "github_actions_environment_variable" "public_entra_id_client_id" {
  repository    = "lerpz-com"
  environment   = var.environment
  variable_name = "PUBLIC_ENTRA_ID_CLIENT_ID"
  value         = var.public_entra_id_client_id
}

resource "github_actions_environment_variable" "public_entra_id_scope" {
  repository    = "lerpz-com"
  environment   = var.environment
  variable_name = "PUBLIC_ENTRA_ID_SCOPE"
  value         = var.public_entra_id_scope
}

resource "github_actions_environment_variable" "public_entra_id_redirect_uri" {
  repository    = "lerpz-com"
  environment   = var.environment
  variable_name = "PUBLIC_ENTRA_ID_REDIRECT_URI"
  value         = var.public_entra_id_redirect_uri
}

resource "github_actions_environment_variable" "public_entra_id_logout_uri" {
  repository    = "lerpz-com"
  environment   = var.environment
  variable_name = "PUBLIC_ENTRA_ID_LOGOUT_URI"
  value         = var.public_entra_id_logout_uri
}
