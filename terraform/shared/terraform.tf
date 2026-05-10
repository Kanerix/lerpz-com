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
  # FIRST-TIME SETUP (two steps):
  #
  #   Step 1 — Comment out the backend block below. Run:
  #
  #     terraform init
  #     terraform apply \
  #       -target=azurerm_resource_group.ext \
  #       -target=azurerm_storage_account.tfstate \
  #       -target=azurerm_storage_container.tfstate
  #
  #   Step 2 — Uncomment the backend block below, then run:
  #
  #     terraform init -migrate-state
  #
  # NORMAL WORKFLOW:
  #
  #   terraform init
  #   terraform apply -var-file=envs/shared.tfvars

  backend "azurerm" {
    resource_group_name  = "lerpz-rg-ext"
    storage_account_name = "lerpztfstate"
    container_name       = "tfstate"
    key                  = "shared.tfstate"
  }
}
