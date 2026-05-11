resource "github_repository" "lerpz_core" {
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
  for_each    = toset(local.environments)
  repository  = github_repository.lerpz_core.name
  environment = each.key
}

resource "github_actions_environment_variable" "azure_client_id" {
  for_each      = toset(local.environments)
  repository    = github_repository.lerpz_core.name
  environment   = each.key
  variable_name = "AZURE_CLIENT_ID"
  value         = azurerm_user_assigned_identity.deployer.client_id

  depends_on = [github_repository_environment.env]
}

resource "github_actions_environment_variable" "azure_tenant_id" {
  for_each      = toset(local.environments)
  repository    = github_repository.lerpz_core.name
  environment   = each.key
  variable_name = "AZURE_TENANT_ID"
  value         = data.azurerm_client_config.current.tenant_id

  depends_on = [github_repository_environment.env]
}

resource "github_actions_environment_variable" "azure_subscription_id" {
  for_each      = toset(local.environments)
  repository    = github_repository.lerpz_core.name
  environment   = each.key
  variable_name = "AZURE_SUBSCRIPTION_ID"
  value         = data.azurerm_subscription.current.subscription_id

  depends_on = [github_repository_environment.env]
}

resource "github_actions_environment_variable" "azure_resource_group" {
  for_each      = toset(local.environments)
  repository    = github_repository.lerpz_core.name
  environment   = each.key
  variable_name = "AZURE_RESOURCE_GROUP"
  value         = local.env_config[each.key].resource_group

  depends_on = [github_repository_environment.env]
}

resource "github_actions_environment_variable" "azure_container_app_name" {
  for_each      = toset(local.environments)
  repository    = github_repository.lerpz_core.name
  environment   = each.key
  variable_name = "AZURE_CONTAINER_APP_NAME"
  value         = local.env_config[each.key].container_app

  depends_on = [github_repository_environment.env]
}

resource "github_actions_variable" "acr_login_server" {
  repository    = github_repository.lerpz_core.name
  variable_name = "ACR_LOGIN_SERVER"
  value         = azurerm_container_registry.lerpz.login_server
}

resource "azurerm_federated_identity_credential" "deployer_env" {
  for_each = toset(local.environments)

  name                      = "github-env-${each.key}"
  user_assigned_identity_id = azurerm_user_assigned_identity.deployer.id

  audience = ["api://AzureADTokenExchange"]
  issuer   = "https://token.actions.githubusercontent.com"
  subject  = "repo:${local.github_org}/${github_repository.lerpz_core.name}:environment:${each.key}"
}
