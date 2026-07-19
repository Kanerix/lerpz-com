<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { Input } from "@lerpz/ui/components/input";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
} from "@lerpz/ui/components/select";
import { setMode, userPrefersMode } from "mode-watcher";
import { toast } from "svelte-sonner";
import { msalStore } from "$lib/auth/msal.svelte.js";
import UserAvatar from "$lib/components/avatar/UserAvatar.svelte";

// DRAFT: this page scaffolds the account settings surface. Preferences are held
// in local state and "Save" just shows a toast so the layout can be reviewed.
// Persisting these should call the profile/preferences API once it exists; the
// theme selector is already wired to mode-watcher and takes effect immediately.

type ThemePref = "light" | "dark" | "system";

const account = $derived(msalStore.activeAccount);

const themeOptions = [
    { value: "light", label: "Light" },
    { value: "dark", label: "Dark" },
    { value: "system", label: "System" },
];
const themeLabel = $derived(
    themeOptions.find((t) => t.value === $userPrefersMode)?.label ?? "System",
);

interface Toggle {
    id: string;
    title: string;
    description: string;
    enabled: boolean;
}

let notifications = $state<Toggle[]>([
    {
        id: "product",
        title: "Product updates",
        description: "New tools, models and features as they ship.",
        enabled: true,
    },
    {
        id: "activity",
        title: "Activity digest",
        description: "A weekly summary of your generations and agents.",
        enabled: false,
    },
    {
        id: "security",
        title: "Security alerts",
        description: "Sign-ins from new devices and role changes.",
        enabled: true,
    },
]);

let saving = $state(false);

function setTheme(value: string) {
    setMode(value as ThemePref);
}

async function save(event: SubmitEvent) {
    event.preventDefault();
    if (saving) return;
    saving = true;
    try {
        // TODO: await updatePreferences({ notifications }) once the
        // preferences endpoint is available.
        await new Promise((resolve) => setTimeout(resolve, 600));
        toast.success("Settings saved");
    } catch (err) {
        toast.error("Couldn't save settings", {
            description:
                err instanceof Error ? err.message : "Please try again.",
        });
    } finally {
        saving = false;
    }
}
</script>

<ScrollArea class="h-full" orientation="vertical">
<div class="mx-auto flex w-full max-w-2xl flex-col gap-8 px-4 py-8">
  <header class="flex items-center gap-3">
    <Icon icon="fa6-solid:gear" class="size-6 text-primary" />
    <div class="flex flex-col gap-1">
      <h1 class="text-2xl font-semibold tracking-tight">Settings</h1>
      <p class="text-sm text-muted-foreground">
        Manage your profile, appearance and notification preferences.
      </p>
    </div>
  </header>

  <form class="flex flex-col gap-8" onsubmit={save}>
    <!-- Profile -->
    <section class="flex flex-col gap-4">
      <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
        Profile
      </h2>

      <div class="flex items-center gap-4">
        <UserAvatar size="lg" />
        <div class="flex min-w-0 flex-col">
          <span class="truncate text-sm font-medium">{account?.name}</span>
          <span class="truncate text-xs text-muted-foreground">{account?.username}</span>
        </div>
      </div>

      <div class="flex flex-col gap-2">
        <label for="email" class="text-sm font-medium">Email</label>
        <Input id="email" value={account?.username ?? ""} disabled readonly />
        <p class="text-xs text-muted-foreground">
          Your profile is managed by your organisation and can't be changed here.
        </p>
      </div>
    </section>

    <!-- Appearance -->
    <section class="flex flex-col gap-4">
      <div class="flex flex-col gap-1">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
          Appearance
        </h2>
        <p class="text-xs text-muted-foreground">
          Choose how Lerpz looks. System follows your device setting.
        </p>
      </div>

      <div class="flex flex-col gap-2">
        <span class="text-sm font-medium">Theme</span>
        <Select
          items={themeOptions}
          value={$userPrefersMode}
          onValueChange={(v) => {
            if (v) setTheme(v);
          }}
        >
          <SelectTrigger class="sm:w-60">{themeLabel}</SelectTrigger>
          <SelectContent>
            {#each themeOptions as option (option.value)}
              <SelectItem value={option.value}>{option.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </section>

    <!-- Notifications -->
    <section class="flex flex-col gap-4">
      <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
        Notifications
      </h2>

      <div class="flex flex-col divide-y rounded-xl border">
        {#each notifications as item (item.id)}
          <label class="flex cursor-pointer items-start justify-between gap-4 p-4">
            <span class="flex flex-col gap-0.5">
              <span class="text-sm font-medium">{item.title}</span>
              <span class="text-xs text-muted-foreground">{item.description}</span>
            </span>
            <button
              type="button"
              role="switch"
              aria-checked={item.enabled}
              aria-label={item.title}
              onclick={() => (item.enabled = !item.enabled)}
              class="relative mt-0.5 inline-flex h-6 w-11 shrink-0 items-center rounded-full transition-colors
                {item.enabled ? 'bg-primary' : 'bg-input'}"
            >
              <span
                class="inline-block size-5 translate-x-0.5 rounded-full bg-background shadow transition-transform
                  {item.enabled ? 'translate-x-[22px]' : ''}"
              ></span>
            </button>
          </label>
        {/each}
      </div>
    </section>

    <!-- Danger zone -->
    <section class="flex flex-col gap-4">
      <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
        Account
      </h2>
      <div class="flex flex-col gap-3 rounded-xl border border-destructive/40 p-4 sm:flex-row sm:items-center sm:justify-between">
        <div class="flex flex-col gap-0.5">
          <span class="text-sm font-medium">Sign out</span>
          <span class="text-xs text-muted-foreground">
            End your session on this device.
          </span>
        </div>
        <Button
          type="button"
          variant="outline"
          class="text-destructive hover:text-destructive"
          onclick={() => msalStore.logoutRedirect()}
        >
          <Icon icon="fa6-solid:right-from-bracket" class="size-4" />
          Sign out
        </Button>
      </div>
    </section>

    <div class="flex items-center justify-end gap-3 border-t pt-6">
      <Button type="submit" disabled={saving}>
        {#if saving}
          <Icon icon="fa6-solid:spinner" class="size-4 animate-spin" />
          Saving…
        {:else}
          <Icon icon="fa6-solid:check" class="size-4" />
          Save changes
        {/if}
      </Button>
    </div>
  </form>
</div>
</ScrollArea>
