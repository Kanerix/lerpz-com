<script lang="ts">
import Icon from "@iconify/svelte";
import { Badge } from "@lerpz/ui/components/badge";
import { Button } from "@lerpz/ui/components/button";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";

// DRAFT: static session data. Wire up to the sessions API
// (`$lib/api/sessions`) so this reflects the live pods backing each agent. Each
// session is a running container attached to the agent's memory volume.

type SessionStatus = "running" | "pending" | "stopped";

type AgentSession = {
    id: string;
    agent: string;
    model: string;
    status: SessionStatus;
    pod: string;
    volume: string;
    messages: number;
    uptime: string;
};

const sessions: AgentSession[] = [
    {
        id: "sess-1a2b",
        agent: "research-assistant",
        model: "GPT-4o",
        status: "running",
        pod: "research-assistant-7d4f9-h2k9",
        volume: "mem-research-01",
        messages: 128,
        uptime: "3h 12m",
    },
    {
        id: "sess-3c4d",
        agent: "support-triage",
        model: "Claude 3.7 Sonnet",
        status: "running",
        pod: "support-triage-5b8c1-x9m2",
        volume: "mem-support-01",
        messages: 42,
        uptime: "58m",
    },
    {
        id: "sess-5e6f",
        agent: "code-reviewer",
        model: "Llama 3.1 70B",
        status: "pending",
        pod: "code-reviewer-9a2e7-p1q4",
        volume: "mem-code-01",
        messages: 0,
        uptime: "—",
    },
    {
        id: "sess-7g8h",
        agent: "data-analyst",
        model: "GPT-4o",
        status: "stopped",
        pod: "data-analyst-2f6d3-t8n5",
        volume: "mem-analytics-01",
        messages: 310,
        uptime: "—",
    },
];

const statusMeta: Record<
    SessionStatus,
    { label: string; dot: string; variant: "default" | "secondary" | "outline" }
> = {
    running: { label: "Running", dot: "bg-green-500", variant: "secondary" },
    pending: { label: "Pending", dot: "bg-amber-500", variant: "outline" },
    stopped: {
        label: "Stopped",
        dot: "bg-muted-foreground",
        variant: "outline",
    },
};
</script>

<ScrollArea class="h-full" orientation="vertical">
<div class="mx-auto flex w-full max-w-5xl flex-col gap-8 px-4 py-8">
  <header class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
    <div class="flex flex-col gap-1">
      <h1 class="text-2xl font-semibold tracking-tight">Sessions</h1>
      <p class="text-sm text-muted-foreground">
        Live agent containers. Open one to chat, or stop it to release its pod.
      </p>
    </div>
    <Button href="/ai/agents">
      <Icon icon="fa6-solid:plus" class="size-3.5" />
      New agent
    </Button>
  </header>

  {#if sessions.length === 0}
    <div class="flex flex-col items-center gap-3 rounded-xl border border-dashed border-border px-4 py-16 text-center">
      <Icon icon="fa6-solid:robot" class="size-8 text-muted-foreground/60" />
      <div class="flex flex-col gap-1">
        <p class="text-base font-medium">No active sessions</p>
        <p class="text-sm text-muted-foreground">
          Deploy an agent to start a session.
        </p>
      </div>
      <Button href="/ai/agents" variant="outline" size="sm" class="mt-1">
        Deploy an agent
      </Button>
    </div>
  {:else}
    <div class="flex flex-col gap-3">
      {#each sessions as session (session.id)}
        {@const meta = statusMeta[session.status]}
        <div class="flex flex-col gap-4 rounded-xl border border-border bg-card p-4 sm:flex-row sm:items-center sm:justify-between">
          <div class="flex min-w-0 flex-col gap-1">
            <div class="flex items-center gap-2">
              <span class="size-2 shrink-0 rounded-full {meta.dot}"></span>
              <span class="truncate font-medium">{session.agent}</span>
              <Badge variant={meta.variant}>{meta.label}</Badge>
            </div>
            <div class="flex flex-wrap items-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
              <span class="inline-flex items-center gap-1.5">
                <Icon icon="fa6-solid:microchip" class="size-3" />
                {session.model}
              </span>
              <span class="inline-flex items-center gap-1.5">
                <Icon icon="fa6-solid:database" class="size-3" />
                {session.volume}
              </span>
              <span class="inline-flex items-center gap-1.5">
                <Icon icon="fa6-solid:cube" class="size-3" />
                <span class="truncate font-mono">{session.pod}</span>
              </span>
            </div>
          </div>

          <div class="flex items-center gap-4">
            <div class="hidden text-right sm:block">
              <p class="text-sm font-medium">{session.messages}</p>
              <p class="text-xs text-muted-foreground">messages</p>
            </div>
            <div class="hidden text-right sm:block">
              <p class="text-sm font-medium">{session.uptime}</p>
              <p class="text-xs text-muted-foreground">uptime</p>
            </div>
            <div class="flex items-center gap-2">
              {#if session.status === "running"}
                <Button href="/ai/chats" size="sm">
                  <Icon icon="fa6-regular:message" class="size-3.5" />
                  Chat
                </Button>
              {:else}
                <Button size="sm" disabled>
                  <Icon icon="fa6-regular:message" class="size-3.5" />
                  Chat
                </Button>
              {/if}
              <Button variant="outline" size="sm" disabled={session.status === "stopped"}>
                {session.status === "stopped" ? "Stopped" : "Stop"}
              </Button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
</ScrollArea>
