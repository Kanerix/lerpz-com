<script lang="ts">
import Icon from "@iconify/svelte";
import { Badge } from "@lerpz/ui/components/badge";
import { Button } from "@lerpz/ui/components/button";
import { Input } from "@lerpz/ui/components/input";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { toast } from "svelte-sonner";

// DRAFT: static marketplace catalogue. Wire up to a templates endpoint so
// "Deploy" hands a template off to the new-agent flow, pre-filling the model,
// memory volume size and system prompt.

type AgentTemplate = {
    id: string;
    name: string;
    description: string;
    icon: string;
    category: string;
    author: string;
    installs: number;
    recommendedModel: string;
    tags: string[];
    featured?: boolean;
};

const templates: AgentTemplate[] = [
    {
        id: "research-assistant",
        name: "Research Assistant",
        description:
            "Reads long documents, keeps notes in memory and answers follow-up questions with citations.",
        icon: "fa6-solid:magnifying-glass",
        category: "Productivity",
        author: "lerpz",
        installs: 1284,
        recommendedModel: "GPT-4o",
        tags: ["rag", "memory", "citations"],
        featured: true,
    },
    {
        id: "code-reviewer",
        name: "Code Reviewer",
        description:
            "Reviews pull requests against your conventions and remembers past feedback per repository.",
        icon: "fa6-solid:code",
        category: "Engineering",
        author: "lerpz",
        installs: 942,
        recommendedModel: "Claude 3.7 Sonnet",
        tags: ["git", "review", "memory"],
        featured: true,
    },
    {
        id: "support-triage",
        name: "Support Triage",
        description:
            "Classifies incoming tickets, drafts replies and escalates based on your playbook.",
        icon: "fa6-solid:headset",
        category: "Support",
        author: "community",
        installs: 611,
        recommendedModel: "GPT-4o",
        tags: ["tickets", "routing"],
    },
    {
        id: "data-analyst",
        name: "Data Analyst",
        description:
            "Answers questions over your datasets and caches derived tables in its memory volume.",
        icon: "fa6-solid:chart-line",
        category: "Analytics",
        author: "community",
        installs: 508,
        recommendedModel: "Llama 3.1 70B",
        tags: ["sql", "charts", "memory"],
    },
    {
        id: "meeting-notetaker",
        name: "Meeting Notetaker",
        description:
            "Summarises transcripts into decisions and action items, indexed by project.",
        icon: "fa6-solid:file-lines",
        category: "Productivity",
        author: "community",
        installs: 377,
        recommendedModel: "GPT-4o",
        tags: ["summaries", "tasks"],
    },
    {
        id: "sql-copilot",
        name: "SQL Copilot",
        description:
            "Turns plain-English questions into queries and remembers your schema between sessions.",
        icon: "fa6-solid:database",
        category: "Engineering",
        author: "lerpz",
        installs: 823,
        recommendedModel: "Claude 3.7 Sonnet",
        tags: ["sql", "schema", "memory"],
    },
];

const categories = [
    "All",
    ...Array.from(new Set(templates.map((t) => t.category))).sort(),
];

let search = $state("");
let activeCategory = $state("All");

const normalizedSearch = $derived(search.trim().toLowerCase());

const filtered = $derived(
    templates.filter((t) => {
        const inCategory =
            activeCategory === "All" || t.category === activeCategory;
        if (!inCategory) return false;
        if (!normalizedSearch) return true;
        return (
            t.name.toLowerCase().includes(normalizedSearch) ||
            t.description.toLowerCase().includes(normalizedSearch) ||
            t.tags.some((tag) => tag.includes(normalizedSearch))
        );
    }),
);

const featured = $derived(filtered.filter((t) => t.featured));

function deploy(template: AgentTemplate) {
    // TODO: route to /ai/agents with the template pre-selected.
    toast.success(`Deploying "${template.name}"`, {
        description: `Recommended model: ${template.recommendedModel}`,
    });
}
</script>

<ScrollArea class="h-full" orientation="vertical">
<div class="mx-auto flex w-full max-w-6xl flex-col gap-8 px-4 py-8">
  <header class="flex flex-col gap-1">
    <h1 class="text-2xl font-semibold tracking-tight">Marketplace</h1>
    <p class="text-sm text-muted-foreground">
      Deploy a prebuilt agent in one click. Each template ships with a
      recommended model and its own memory volume.
    </p>
  </header>

  <!-- Controls -->
  <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
    <div class="flex flex-wrap gap-2">
      {#each categories as category (category)}
        <button
          type="button"
          onclick={() => (activeCategory = category)}
          class="rounded-full border px-3 py-1 text-sm transition-colors
            {activeCategory === category
              ? 'border-primary bg-primary text-primary-foreground'
              : 'border-border text-muted-foreground hover:bg-muted'}"
        >
          {category}
        </button>
      {/each}
    </div>
    <div class="relative w-full sm:max-w-xs">
      <Icon
        icon="fa6-solid:magnifying-glass"
        class="pointer-events-none absolute left-3 top-1/2 size-3.5 -translate-y-1/2 text-muted-foreground"
      />
      <Input
        value={search}
        oninput={(e) => (search = e.currentTarget.value)}
        placeholder="Search templates…"
        aria-label="Search templates"
        class="pl-9"
      />
    </div>
  </div>

  {#if filtered.length === 0}
    <div class="flex flex-col items-center gap-3 rounded-xl border border-dashed border-border px-4 py-16 text-center">
      <Icon icon="fa6-solid:store" class="size-8 text-muted-foreground/60" />
      <div class="flex flex-col gap-1">
        <p class="text-base font-medium">No templates found</p>
        <p class="text-sm text-muted-foreground">
          Try a different search or category.
        </p>
      </div>
    </div>
  {:else}
    {#if featured.length > 0 && activeCategory === "All" && !normalizedSearch}
      <section class="flex flex-col gap-3">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
          Featured
        </h2>
        <div class="grid gap-3 sm:grid-cols-2">
          {#each featured as template (template.id)}
            <div class="flex flex-col gap-4 rounded-2xl border border-border bg-gradient-to-br from-primary/5 to-transparent p-5">
              <div class="flex items-start gap-3">
                <span class="flex size-11 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
                  <Icon icon={template.icon} class="size-5" />
                </span>
                <div class="flex min-w-0 flex-col gap-0.5">
                  <div class="flex items-center gap-2">
                    <h3 class="truncate font-semibold">{template.name}</h3>
                    <Badge variant="secondary">Featured</Badge>
                  </div>
                  <p class="text-xs text-muted-foreground">
                    by {template.author} · {template.installs.toLocaleString()} installs
                  </p>
                </div>
              </div>
              <p class="text-sm text-muted-foreground">{template.description}</p>
              <div class="mt-auto flex items-center justify-between gap-3">
                <span class="inline-flex items-center gap-1.5 text-xs text-muted-foreground">
                  <Icon icon="fa6-solid:microchip" class="size-3" />
                  {template.recommendedModel}
                </span>
                <Button size="sm" onclick={() => deploy(template)}>
                  <Icon icon="fa6-solid:rocket" class="size-3.5" />
                  Deploy
                </Button>
              </div>
            </div>
          {/each}
        </div>
      </section>
    {/if}

    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
        {activeCategory === "All" ? "All templates" : activeCategory}
        <span class="ml-1 font-normal text-muted-foreground/70">({filtered.length})</span>
      </h2>
      <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
        {#each filtered as template (template.id)}
          <div class="flex flex-col gap-3 rounded-2xl border border-border bg-card p-5">
            <div class="flex items-start justify-between gap-2">
              <span class="flex size-10 shrink-0 items-center justify-center rounded-xl bg-muted text-foreground">
                <Icon icon={template.icon} class="size-4.5" />
              </span>
              <Badge variant="outline">{template.category}</Badge>
            </div>
            <div class="flex flex-col gap-1">
              <h3 class="font-semibold">{template.name}</h3>
              <p class="line-clamp-3 text-sm text-muted-foreground">
                {template.description}
              </p>
            </div>
            <div class="flex flex-wrap gap-1.5">
              {#each template.tags as tag (tag)}
                <span class="rounded-md bg-muted px-2 py-0.5 text-xs text-muted-foreground">
                  {tag}
                </span>
              {/each}
            </div>
            <div class="mt-auto flex items-center justify-between gap-3 border-t pt-3">
              <span class="inline-flex items-center gap-1.5 text-xs text-muted-foreground">
                <Icon icon="fa6-solid:download" class="size-3" />
                {template.installs.toLocaleString()}
              </span>
              <Button variant="outline" size="sm" onclick={() => deploy(template)}>
                Deploy
              </Button>
            </div>
          </div>
        {/each}
      </div>
    </section>
  {/if}
</div>
</ScrollArea>
