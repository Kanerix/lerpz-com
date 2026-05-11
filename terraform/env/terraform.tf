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
  # The shared module must be applied before any environment.
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
