locals {
  location = "West Europe"

  # Per-environment derived names
  resource_group_name   = "lerpz-${var.environment}-rg"
  container_app_name    = "lerpz-website-${var.environment}"
  container_env_name    = "lerpz-env-${var.environment}"
  runtime_identity_name = "lerpz-runtime-${var.environment}"
  domain                = var.environment == "prod" ? "lerpz.com" : "stag.lerpz.com"

  # Shorthand alias for shared state outputs.
  shared = data.terraform_remote_state.shared.outputs
}
