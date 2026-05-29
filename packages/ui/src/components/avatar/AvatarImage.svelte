<script lang="ts">
import { Avatar } from "@ark-ui/svelte/avatar";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    src = "",
    alt = "",
    ...rest
}: {
    class?: string;
    src?: string;
    alt?: string;
    [key: string]: unknown;
} = $props();
</script>

<Avatar.Image data-slot="avatar-image" class={cn("aspect-square size-full object-cover", className)} {src} {alt} {...rest} />
