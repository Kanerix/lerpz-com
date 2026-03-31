import { Badge } from "@lerpz/ui/components/badge";
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@lerpz/ui/components/card";
import { MdChat, MdImage, MdSearch } from "react-icons/md";

const tools = [
  {
    icon: MdChat,
    title: "AI Chat",
    description:
      "Have conversations with large language models. Ask questions, summarise documents, or draft content — directly in your browser.",
    badge: "Available",
    badgeVariant: "default" as const,
  },
  {
    icon: MdImage,
    title: "Image Generation",
    description:
      "Generate images from text prompts. Useful for mockups, visual references, and creative work without needing external tools.",
    badge: "Beta",
    badgeVariant: "outline" as const,
  },
  {
    icon: MdSearch,
    title: "Semantic Search",
    description:
      "Search internal documents and knowledge bases by meaning rather than exact keywords. Finds relevant content even when phrasing differs.",
    badge: "Coming soon",
    badgeVariant: "secondary" as const,
  },
];

export default function SolutionSection() {
  return (
    <section className="flex flex-col gap-10 py-16 md:py-24">
      <div className="flex flex-col gap-2">
        <h2 className="text-2xl font-semibold tracking-tight">Available tools</h2>
        <p className="text-muted-foreground">
          A set of AI capabilities maintained centrally so everyone in the
          organisation works from the same, up-to-date tooling.
        </p>
      </div>

      <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
        {tools.map((tool) => {
          const Icon = tool.icon;
          return (
            <Card key={tool.title}>
              <CardHeader>
                <div className="mb-3 flex items-center justify-between">
                  <div className="flex size-9 items-center justify-center rounded-lg bg-primary/10 text-primary">
                    <Icon className="size-5" />
                  </div>
                  <Badge variant={tool.badgeVariant}>{tool.badge}</Badge>
                </div>
                <CardTitle className="text-base font-semibold">
                  {tool.title}
                </CardTitle>
                <CardDescription className="text-sm leading-relaxed">
                  {tool.description}
                </CardDescription>
              </CardHeader>
            </Card>
          );
        })}
      </div>
    </section>
  );
}