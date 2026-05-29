<script lang="ts">
import Icon from "@iconify/svelte";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuGroup,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuTrigger,
} from "@lerpz/ui/components/dropdown-menu";
import {
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    useSidebar,
} from "@lerpz/ui/components/sidebar";
import { goto } from "$app/navigation";
import { untrack } from "svelte";

type Mode = {
    variant: "chat" | "image" | "video";
    name: string;
    icon: string;
    plan: string;
    href: string;
};

let { defaultMode, modes }: { defaultMode: Mode; modes: Mode[] } = $props();

const sidebar = useSidebar();
let activeMode = $state<Mode>(untrack(() => defaultMode));
</script>

<SidebarMenu>
  <SidebarMenuItem>
    <DropdownMenu>
      <DropdownMenuTrigger class="w-full">
        <SidebarMenuButton size="lg">
          <div class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
            <Icon icon={activeMode.icon} class="size-4" />
          </div>
          <div class="flex flex-col gap-0.5 leading-none">
            <span class="truncate font-medium">{activeMode.name}</span>
            <span class="truncate text-xs">{activeMode.plan}</span>
          </div>
          <Icon icon="mdi:unfold-more-horizontal" class="ml-auto" />
        </SidebarMenuButton>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        align="start"
        side={sidebar.isMobile ? "bottom" : "right"}
        sideOffset={4}
      >
        <DropdownMenuGroup>
          <DropdownMenuLabel class="text-muted-foreground text-xs">Modes</DropdownMenuLabel>
          {#each modes as mode}
            <DropdownMenuItem
              value={mode.variant}
              onclick={() => { activeMode = mode; goto(mode.href); }}
              class="gap-2 p-2"
            >
              <div class="flex size-6 items-center justify-center rounded-md border">
                <Icon icon={mode.icon} class="size-3.5 shrink-0" />
              </div>
              {mode.name}
            </DropdownMenuItem>
          {/each}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  </SidebarMenuItem>
</SidebarMenu>
