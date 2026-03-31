import { Badge } from "@lerpz/ui/components/badge";
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@lerpz/ui/components/card";
import { Separator } from "@lerpz/ui/components/separator";
import {
  MdAutoAwesome,
  MdChat,
  MdImage,
  MdLock,
  MdSearch,
  MdSpeed,
} from "react-icons/md";

const features = [
  {
    icon: MdChat,
    title: "AI Chat",
    description:
      "Conversational AI powered by the latest large language models. Ask \
      questions, summarise documents, draft content, and get instant answers — \
      right inside your workflow.",
    badge: "Available now",
    badgeVariant: "default" as const,
  },
  {
    icon: MdImage,
    title: "Image Generation",
    description:
      "Turn text prompts into stunning visuals in seconds. Generate product \
      mockups, marketing assets, illustrations, and more without a design \
      team.",
    badge: "Beta",
    badgeVariant: "outline" as const,
  },
  {
    icon: MdSearch,
    title: "Semantic Search",
    description:
      "Search across your organisation's knowledge base with meaning, not just \
      keywords. Surface the right information from documents, wikis, and data \
      sources instantly.",
    badge: "Coming soon",
    badgeVariant: "secondary" as const,
  },
];

const highlights = [
  {
    icon: MdAutoAwesome,
    title: "Curated tool ecosystem",
    description:
      "We hand-pick and integrate the best AI models so you don't have to manage \
      API keys, rate limits, or model updates.",
  },
  {
    icon: MdSpeed,
    title: "Built for speed",
    description:
      "Optimised inference infrastructure means responses in milliseconds, not \
      seconds — even under heavy organisational load.",
  },
  {
    icon: MdLock,
    title: "Enterprise-grade security",
    description:
      "SSO, role-based access control, audit logs, and data residency options \
      come standard. Your data never trains third-party models.",
  },
];

export default function SolutionSection() {
  return (
    <section className="flex flex-col gap-20 py-16 md:py-24">
      {/* Section header */}
      <div className="flex flex-col items-center gap-4 text-center">
        <Badge variant="outline" className="w-fit">
          What we offer
        </Badge>
        <h2 className="max-w-2xl text-3xl font-bold tracking-tight md:text-4xl xl:text-5xl">
          Everything your team needs to work smarter with AI
        </h2>
        <p className="max-w-xl text-muted-foreground md:text-lg">
          One portal. Multiple capabilities. Zero friction. We handle the
          complexity so your team can focus on results.
        </p>
      </div>

      {/* Feature cards */}
      <div className="grid grid-cols-1 gap-6 md:grid-cols-3">
        {features.map((feature) => {
          const Icon = feature.icon;
          return (
            <Card key={feature.title} className="relative">
              <CardHeader>
                <div className="mb-2 flex items-center justify-between">
                  <div className="flex size-10 items-center justify-center rounded-xl bg-primary/10 text-primary">
                    <Icon className="size-5" />
                  </div>
                  <Badge variant={feature.badgeVariant}>{feature.badge}</Badge>
                </div>
                <CardTitle className="text-lg font-semibold">
                  {feature.title}
                </CardTitle>
                <CardDescription className="text-sm leading-relaxed">
                  {feature.description}
                </CardDescription>
              </CardHeader>
            </Card>
          );
        })}
      </div>

      <Separator />

      {/* Why Lerpz highlights */}
      <div className="flex flex-col gap-12">
        <div className="flex flex-col items-center gap-3 text-center">
          <h3 className="text-2xl font-bold tracking-tight md:text-3xl">
            Why Lerpz AI?
          </h3>
          <p className="max-w-xl text-muted-foreground">
            We don't just give you access to AI — we make sure it works safely
            and reliably at scale across your entire organisation.
          </p>
        </div>

        <div className="grid grid-cols-1 gap-8 md:grid-cols-3">
          {highlights.map((item) => {
            const Icon = item.icon;
            return (
              <div key={item.title} className="flex flex-col gap-3">
                <div className="flex size-10 items-center justify-center rounded-xl bg-primary/10 text-primary">
                  <Icon className="size-5" />
                </div>
                <h4 className="font-semibold">{item.title}</h4>
                <p className="text-sm leading-relaxed text-muted-foreground">
                  {item.description}
                </p>
              </div>
            );
          })}
        </div>
      </div>

      {/* Bottom CTA banner */}
      <div className="flex flex-col items-center gap-6 rounded-2xl bg-primary/10 px-8 py-12 text-center ring-1 ring-primary/20">
        <h3 className="text-2xl font-bold md:text-3xl">
          Ready to bring AI to your organisation?
        </h3>
        <p className="max-w-lg text-muted-foreground">
          Join the growing list of teams already using Lerpz AI to automate
          work, accelerate decisions, and build smarter products.
        </p>
        <a
          href="#newsletter"
          className="inline-flex items-center gap-2 rounded-full bg-primary px-6 py-3 text-sm font-semibold text-primary-foreground transition-colors hover:bg-primary/80"
        >
          Request access
        </a>
      </div>
    </section>
  );
}
