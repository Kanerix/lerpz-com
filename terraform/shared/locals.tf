locals {
  location     = "West Europe"
  github_org   = "kanerix"
  environments = ["prod", "stag"]

  env_config = {
    prod = {
      resource_group = "lerpz-prod-rg"
      container_app  = "lerpz-website-prod"
    }
    stag = {
      resource_group = "lerpz-stag-rg"
      container_app  = "lerpz-website-stag"
    }
  }
}
