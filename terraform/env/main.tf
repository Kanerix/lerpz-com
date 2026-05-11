provider "azurerm" {
  features {}
}

provider "github" {
  owner = "Kanerix"
}

# Read outputs from the shared root module.
# The shared module must be fully applied before planning or applying any environment.
data "terraform_remote_state" "shared" {
  backend = "azurerm"

  config = {
    resource_group_name  = "lerpz-rg-ext"
    storage_account_name = "lerpztfstate"
    container_name       = "tfstate"
    key                  = "shared.tfstate"
  }
}

resource "azurerm_resource_group" "lerpz" {
  name     = local.resource_group_name
  location = local.location
}
