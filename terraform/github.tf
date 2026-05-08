# GitHub Repository
#
# The repository, environments, variables, and federated credentials are all
# managed in the prod state only. The stag state does not create or modify
# any GitHub resources.

locals {
  env_config = var.environment == "prod" ? {
    prod = {
      resource_group = "lerpz-prod-rg"
      container_app  = "lerpz-website-prod"
      client_id      = azurerm_user_assigned_identity.deployer[0].client_id
    }
    stag = {
      resource_group = "lerpz-stag-rg"
      container_app  = "lerpz-website-stag"
      client_id      = azurerm_user_assigned_identity.deployer[0].client_id
    }
  } : {}
}

resource "github_repository" "lerpz_core" {
  count       = var.environment == "prod" ? 1 : 0
  name        = "lerpz-com"
  description = "General-purpose monorepo for the Lerpz organisation."
  visibility  = "private"

  has_issues   = true
  has_projects = false
  has_wiki     = false

  delete_branch_on_merge = true

  squash_merge_commit_title   = "PR_TITLE"
  squash_merge_commit_message = "BLANK"
}

resource "github_repository_environment" "env" {
  for_each    = var.environment == "prod" ? toset(local.environments) : toset([])
  repository  = github_repository.lerpz_core[0].name
  environment = each.key
}

resource "github_actions_environment_variable" "azure_client_id" {
  for_each      = var.environment == "prod" ? toset(local.environments) : toset([])
  repository    = github_repository.lerpz_core[0].name
  environment   = each.key
  variable_name = "AZURE_CLIENT_ID"
  value         = local.env_config[each.key].client_id

  depends_on = [github_repository_environment.env]
}

resource "github_actions_environment_variable" "azure_tenant_id" {
  for_each      = var.environment == "prod" ? toset(local.environments) : toset([])
  repository    = github_repository.lerpz_core[0].name
  environment   = each.key
  variable_name = "AZURE_TENANT_ID"
  value         = data.azurerm_client_config.current.tenant_id

  depends_on = [github_repository_environment.env]
}

resource "github_actions_environment_variable" "azure_subscription_id" {
  for_each      = var.environment == "prod" ? toset(local.environments) : toset([])
  repository    = github_repository.lerpz_core[0].name
  environment   = each.key
  variable_name = "AZURE_SUBSCRIPTION_ID"
  value         = data.azurerm_subscription.current.subscription_id

  depends_on = [github_repository_environment.env]
}

resource "github_actions_environment_variable" "azure_resource_group" {
  for_each      = var.environment == "prod" ? toset(local.environments) : toset([])
  repository    = github_repository.lerpz_core[0].name
  environment   = each.key
  variable_name = "AZURE_RESOURCE_GROUP"
  value         = local.env_config[each.key].resource_group

  depends_on = [github_repository_environment.env]
}

resource "github_actions_environment_variable" "azure_container_app_name" {
  for_each      = var.environment == "prod" ? toset(local.environments) : toset([])
  repository    = github_repository.lerpz_core[0].name
  environment   = each.key
  variable_name = "AZURE_CONTAINER_APP_NAME"
  value         = local.env_config[each.key].container_app

  depends_on = [github_repository_environment.env]
}

# ---------------------------------------------------------------------------
# Repository-level Actions variables
# ---------------------------------------------------------------------------
# These are not environment-specific and are available to all workflows.

resource "github_actions_variable" "acr_login_server" {
  count         = var.environment == "prod" ? 1 : 0
  repository    = github_repository.lerpz_core[0].name
  variable_name = "ACR_LOGIN_SERVER"
  value         = local.acr.login_server
}

# Federated Identity Credential
#
# Allows GitHub Actions workflows in this repository to authenticate as the
# deployer managed identity via OIDC — no secrets or passwords required.

resource "azurerm_federated_identity_credential" "deployer_pr" {
  count                     = var.environment == "prod" ? 1 : 0
  name                      = "github-pr"
  user_assigned_identity_id = local.deployer.id

  audience = ["api://AzureADTokenExchange"]
  issuer   = "https://token.actions.githubusercontent.com"
  subject  = "repo:${local.github_org}/${github_repository.lerpz_core[0].name}:pull_request"
}

resource "azurerm_federated_identity_credential" "deployer_env" {
  for_each = var.environment == "prod" ? toset(local.environments) : toset([])

  name                      = "github-env-${each.key}"
  user_assigned_identity_id = local.deployer.id

  audience = ["api://AzureADTokenExchange"]
  issuer   = "https://token.actions.githubusercontent.com"
  subject  = "repo:${local.github_org}/${github_repository.lerpz_core[0].name}:environment:${each.key}"
}
