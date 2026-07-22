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
        icon: "fa6-regular:comment",
        title: "AI Chat",
        description:
            "Have conversations with large language models. Ask questions, \
            summarise documents, or draft content — with your full chat history \
            a click away.",
        badge: "Available",
        badgeVariant: "default" as const,
    },
    {
        icon: "fa6-regular:image",
        title: "Image Generation",
        description:
            "Generate images from text prompts and keep them organised in your \
            personal gallery — ideal for mockups, visual references, and \
            creative work.",
        badge: "Available",
        badgeVariant: "default" as const,
    },
    {
        icon: "fa6-solid:magnifying-glass",
        title: "Image Analysis",
        description:
            "Upload your own images and let AI describe, analyse, and answer \
            questions about what's in them.",
        badge: "New",
        badgeVariant: "secondary" as const,
    },
    {
        icon: "fa6-solid:video",
        title: "Video Generation",
        description:
            "Turn prompts into short videos and browse everything you've created \
            from a dedicated gallery.",
        badge: "Beta",
        badgeVariant: "outline" as const,
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

  <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
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
