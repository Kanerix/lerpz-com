variable "environment" {
  description = "Deployment environment. Either 'prod' or 'stag'."
  type        = string

  validation {
    condition     = contains(["prod", "stag"], var.environment)
    error_message = "environment must be 'prod' or 'stag'."
  }
}

variable "next_public_api_url" {
  description = "Public URL of the backend API (NEXT_PUBLIC_API_URL)."
  type        = string
}

variable "next_public_entra_id_tenant_id" {
  description = "Entra ID (AAD) tenant ID (NEXT_PUBLIC_ENTRA_ID_TENANT_ID)."
  type        = string
}

variable "next_public_entra_id_client_id" {
  description = "Entra ID (AAD) client/app ID (NEXT_PUBLIC_ENTRA_ID_CLIENT_ID)."
  type        = string
}

variable "next_public_entra_id_scope" {
  description = "OAuth2 scope requested by the portal (NEXT_PUBLIC_ENTRA_ID_SCOPE)."
  type        = string
}

variable "next_public_entra_id_redirect_uri" {
  description = "Post-login redirect URI (NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI)."
  type        = string
}

variable "next_public_entra_id_logout_uri" {
  description = "Post-logout redirect URI (NEXT_PUBLIC_ENTRA_ID_LOGOUT_URI)."
  type        = string
}

variable "container_image" {
  description = <<-EOT
    Container image to run in the Container App.
    Defaults to a Microsoft hello-world placeholder until a real image is
    pushed to ACR and this value is overridden
    (e.g. via a tfvars file or CI/CD pipeline variable).
  EOT
  type        = string
  default     = "mcr.microsoft.com/azuredocs/aci-helloworld:latest"
}
