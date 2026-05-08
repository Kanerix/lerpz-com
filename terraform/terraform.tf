terraform {
  required_version = ">= 1.6.0"

  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 4.0"
    }
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
  }

  # Remote State — Azure Blob Storage
  #
  # Each environment uses its own state file, selected at init time via a
  # backend config partial file.
  #
  # FIRST-TIME SETUP (prod only, two steps):
  #
  #   Step 1 — Backend is commented out below. Run:
  #
  #     terraform init
  #     terraform apply -var-file=envs/prod.tfvars \
  #                     -target=azurerm_resource_group.ext \
  #                     -target=azurerm_storage_account.tfstate \
  #                     -target=azurerm_storage_container.tfstate
  #
  #   Step 2 — Uncomment the backend block below, then run:
  #
  #     terraform init -migrate-state -backend-config=envs/prod.tfbackend
  #
  # NORMAL WORKFLOW:
  #
  #   # prod
  #   terraform init -backend-config=envs/prod.tfbackend
  #   terraform apply -var-file=envs/prod.tfvars
  #
  #   # stag
  #   terraform init -backend-config=envs/stag.tfbackend
  #   terraform apply -var-file=envs/stag.tfvars

  backend "azurerm" {
    resource_group_name  = "lerpz-rg-ext"
    storage_account_name = "lerpztfstate"
    container_name       = "tfstate"
    # key is supplied via -backend-config=envs/<env>.tfbackend
  }
}
