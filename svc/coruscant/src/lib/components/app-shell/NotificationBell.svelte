<script lang="ts">
import Icon from "@iconify/svelte";
import { buttonVariants } from "@lerpz/ui/components/button";
import {
    Popover,
    PopoverContent,
    PopoverPositioner,
    PopoverTrigger,
} from "@lerpz/ui/components/popover";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { cn } from "@lerpz/ui/lib/utils";
import { goto } from "$app/navigation";
import {
    type AppNotification,
    notificationStore,
} from "$lib/notifications/notifications.svelte.js";

let open = $state(false);

const notifications = $derived(notificationStore.notifications);
const unreadCount = $derived(notificationStore.unreadCount);
const badgeLabel = $derived(unreadCount > 9 ? "9+" : String(unreadCount));

const relativeTime = new Intl.RelativeTimeFormat(undefined, {
    numeric: "auto",
});

const DIVISIONS: { amount: number; unit: Intl.RelativeTimeFormatUnit }[] = [
    { amount: 60, unit: "seconds" },
    { amount: 60, unit: "minutes" },
    { amount: 24, unit: "hours" },
    { amount: 7, unit: "days" },
    { amount: 4.34524, unit: "weeks" },
    { amount: 12, unit: "months" },
    { amount: Number.POSITIVE_INFINITY, unit: "years" },
];

function formatRelative(timestamp: number): string {
    let duration = (timestamp - Date.now()) / 1000;
    for (const division of DIVISIONS) {
        if (Math.abs(duration) < division.amount) {
            return relativeTime.format(Math.round(duration), division.unit);
        }
        duration /= division.amount;
    }
    return "";
}

function handleSelect(notification: AppNotification) {
    notificationStore.markRead(notification.id);
    if (notification.href) {
        open = false;
        goto(notification.href);
    }
}
</script>

<Popover {open} onOpenChange={(e: { open: boolean }) => (open = e.open)} align="end">
  <PopoverTrigger
    class={cn(
      buttonVariants({ variant: "ghost", size: "icon" }),
      "relative",
    )}
    aria-label={unreadCount > 0
      ? `Notifications, ${unreadCount} unread`
      : "Notifications"}
  >
    <Icon icon="fa6-solid:bell" class="h-5 w-5" />
    {#if unreadCount > 0}
      <span
        class="absolute -right-0.5 -top-0.5 flex h-4 min-w-4 items-center justify-center rounded-full bg-primary px-1 text-[10px] font-semibold leading-none text-primary-foreground"
      >
        {badgeLabel}
      </span>
    {/if}
  </PopoverTrigger>

  <PopoverPositioner>
    <PopoverContent class="flex w-80 flex-col overflow-hidden p-0">
      <div class="flex items-center justify-between gap-2 border-b px-4 py-3">
        <span class="text-sm font-semibold">Notifications</span>
        {#if unreadCount > 0}
          <button
            type="button"
            class="text-xs font-medium text-muted-foreground transition-colors hover:text-foreground"
            onclick={() => notificationStore.markAllRead()}
          >
            Mark all read
          </button>
        {/if}
      </div>

      {#if notifications.length === 0}
        <div class="flex flex-col items-center gap-2 px-4 py-10 text-center">
          <Icon
            icon="fa6-regular:bell"
            class="size-6 text-muted-foreground"
          />
          <p class="text-sm text-muted-foreground">You're all caught up.</p>
        </div>
      {:else}
        <ScrollArea class="max-h-96" orientation="vertical">
          <ul class="flex flex-col divide-y">
            {#each notifications as item (item.id)}
              <li class="relative">
                <button
                  type="button"
                  onclick={() => handleSelect(item)}
                  class={cn(
                    "flex w-full items-start gap-3 px-4 py-3 text-left transition-colors hover:bg-accent/50",
                    !item.read && "bg-accent/30",
                  )}
                >
                  <span
                    class="mt-0.5 flex size-8 shrink-0 items-center justify-center rounded-full bg-muted text-muted-foreground"
                  >
                    <Icon icon={item.icon ?? "fa6-solid:bell"} class="size-3.5" />
                  </span>
                  <span class="flex min-w-0 flex-1 flex-col gap-0.5">
                    <span class="flex items-center gap-2">
                      <span class="truncate text-sm font-medium">{item.title}</span>
                      {#if !item.read}
                        <span
                          class="size-2 shrink-0 rounded-full bg-primary"
                          aria-label="Unread"
                        ></span>
                      {/if}
                    </span>
                    {#if item.body}
                      <span class="text-xs text-muted-foreground">{item.body}</span>
                    {/if}
                    <span class="text-[11px] text-muted-foreground">
                      {formatRelative(item.createdAt)}
                    </span>
                  </span>
                </button>
              </li>
            {/each}
          </ul>
        </ScrollArea>
      {/if}
    </PopoverContent>
  </PopoverPositioner>
</Popover>
