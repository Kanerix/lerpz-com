variable "environment" {
  description = "Deployment environment. Either 'prod' or 'stag'."
  type        = string

  validation {
    condition     = contains(["prod", "stag"], var.environment)
    error_message = "environment must be 'prod' or 'stag'."
  }
}

variable "public_api_url" {
  description = "Public URL of the backend API (PUBLIC_API_URL)."
  type        = string
}

variable "public_entra_id_tenant_id" {
  description = "Entra ID (AAD) tenant ID (PUBLIC_ENTRA_ID_TENANT_ID)."
  type        = string
}

variable "public_entra_id_client_id" {
  description = "Entra ID (AAD) client/app ID (PUBLIC_ENTRA_ID_CLIENT_ID)."
  type        = string
}

variable "public_entra_id_scope" {
  description = "OAuth2 scope requested by the portal (PUBLIC_ENTRA_ID_SCOPE)."
  type        = string
}

variable "public_entra_id_redirect_uri" {
  description = "Post-login redirect URI (PUBLIC_ENTRA_ID_REDIRECT_URI)."
  type        = string
}

variable "public_entra_id_logout_uri" {
  description = "Post-logout redirect URI (PUBLIC_ENTRA_ID_LOGOUT_URI)."
  type        = string
}

variable "container_image" {
  description = <<-EOT
    Container image to run in the Container App.
    Defaults to the official Azure Container Apps hello-world placeholder
    (port 80) until a real image is pushed to ACR and this value is
    overridden (e.g. via a tfvars file or CI/CD pipeline variable).
  EOT
  type    = string
  default = "mcr.microsoft.com/azuredocs/containerapps-helloworld:latest"
}

variable "container_port" {
  description = <<-EOT
    Port the container listens on. Defaults to 80 to match the placeholder
    image. Override to 3000 (or whatever your app uses) in each env tfvars.
  EOT
  type    = number
  default = 80
}
