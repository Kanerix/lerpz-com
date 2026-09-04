<script lang="ts">
import Icon from "@iconify/svelte";
import {
    Avatar,
    AvatarFallback,
    AvatarImage,
} from "@lerpz/ui/components/avatar";
import { cn } from "@lerpz/ui/lib/utils";
import { modelFamilyLogoForTheme } from "$lib/ai/model-logo.svelte.js";

let {
    family = null,
    label,
    size = "sm",
    class: className,
}: {
    family?: string | null;
    label?: string;
    size?: "sm" | "default" | "lg";
    class?: string;
} = $props();

const logo = $derived(modelFamilyLogoForTheme(family));
</script>

<Avatar {size} class={cn("shrink-0", className)}>
  {#if logo}
    <AvatarImage src={logo} alt={label ?? "AI"} class="object-contain p-1" />
  {/if}
  <AvatarFallback>
    <Icon icon="fa6-solid:robot" class="size-3.5" />
  </AvatarFallback>
</Avatar>
