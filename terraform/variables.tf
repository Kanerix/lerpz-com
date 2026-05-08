variable "environment" {
  description = "Deployment environment. Either 'prod' or 'stag'."
  type        = string

  validation {
    condition     = contains(["prod", "stag"], var.environment)
    error_message = "environment must be 'prod' or 'stag'."
  }
}

# ACR names must be globally unique across all of Azure.
# "lerpzacr" is the chosen name — verify availability before first apply.
variable "acr_name" {
  description = "Globally unique name for the Azure Container Registry (shared across environments)."
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
