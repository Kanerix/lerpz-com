<script lang="ts">
import CallToAction from "$lib/components/CallToAction.svelte";
import CapabilitiesSection from "$lib/components/CapabilitiesSection.svelte";
import HeroSection from "$lib/components/HeroSection.svelte";
import PlatformSection from "$lib/components/PlatformSection.svelte";
import PortfolioSection from "$lib/components/PortfolioSection.svelte";
import { site } from "$lib/config.js";
</script>

<svelte:head>
  <title>Lerpz</title>
  <meta name="description" content={site.description} />
</svelte:head>

<HeroSection />
<CapabilitiesSection />
<PlatformSection />
<PortfolioSection />
<CallToAction />
