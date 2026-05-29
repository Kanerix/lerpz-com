<script lang="ts">
import Icon from "@iconify/svelte";
import { Badge } from "@lerpz/ui/components/badge";
import {
    Card,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@lerpz/ui/components/card";

const tools = [
    {
        icon: "mdi:chat-outline",
        title: "AI Chat",
        description:
            "Have conversations with large language models. Ask questions, summarise documents, or draft content — directly in your browser.",
        badge: "Available",
        badgeVariant: "default" as const,
    },
    {
        icon: "mdi:image-outline",
        title: "Image Generation",
        description:
            "Generate images from text prompts. Useful for mockups, visual references, and creative work without needing external tools.",
        badge: "Beta",
        badgeVariant: "outline" as const,
    },
    {
        icon: "mdi:magnify",
        title: "Semantic Search",
        description:
            "Search internal documents and knowledge bases by meaning rather than exact keywords.",
        badge: "Coming soon",
        badgeVariant: "secondary" as const,
    },
];
</script>

<section class="flex flex-col gap-10 py-16 md:py-24">
  <div class="flex flex-col gap-2">
    <h2 class="text-2xl font-semibold tracking-tight">Available tools</h2>
    <p class="text-muted-foreground">
      A set of AI capabilities maintained centrally so everyone in the organisation works from the same, up-to-date tooling.
    </p>
  </div>

  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#each tools as tool}
      <Card>
        <CardHeader>
          <div class="mb-3 flex items-center justify-between">
            <div class="flex size-9 items-center justify-center rounded-lg bg-primary/10 text-primary">
              <Icon icon={tool.icon} class="size-5" />
            </div>
            <Badge variant={tool.badgeVariant}>{tool.badge}</Badge>
          </div>
          <CardTitle class="text-base font-semibold">{tool.title}</CardTitle>
          <CardDescription class="text-sm leading-relaxed">{tool.description}</CardDescription>
        </CardHeader>
      </Card>
    {/each}
  </div>
</section>
