import Image from "next/image";
import Link from "next/link";
import { MdFavorite } from "react-icons/md";
import { SiGithub } from "react-icons/si";

const footerLinks = {
  Product: [
    { label: "Features", href: "#features" },
    { label: "Pricing", href: "#pricing" },
    { label: "Changelog", href: "/changelog" },
    { label: "Roadmap", href: "/roadmap" },
  ],
  Developers: [
    { label: "Documentation", href: "/docs" },
    { label: "API Reference", href: "/docs/api" },
    { label: "Status", href: "/status" },
    { label: "GitHub", href: "https://github.com/lerpz" },
  ],
  Company: [
    { label: "About", href: "/about" },
    { label: "Blog", href: "/blog" },
    { label: "Careers", href: "/careers" },
    { label: "Contact", href: "/contact" },
  ],
  Legal: [
    { label: "Privacy Policy", href: "/legal/privacy" },
    { label: "Terms of Service", href: "/legal/terms" },
    { label: "Cookie Policy", href: "/legal/cookies" },
    { label: "Security", href: "/legal/security" },
  ],
};

const socialLinks = [
  {
    label: "GitHub",
    href: "https://github.com/lerpz",
    icon: SiGithub,
  },
];

export default function Footer() {
  const year = new Date().getFullYear();

  return (
    <footer className="border-t border-border/60 bg-card/50">
      <div className="mx-auto max-w-[1024px] px-4 py-12 md:py-16">
        {/* Top: brand + links grid */}
        <div className="grid grid-cols-2 gap-10 md:grid-cols-5">
          {/* Brand column */}
          <div className="col-span-2 flex flex-col gap-4 md:col-span-1">
            <Link
              href="/"
              className="flex items-center gap-2.5 font-bold text-foreground transition-opacity hover:opacity-80"
            >
              <Image
                src="/lerpz.svg"
                alt="Lerpz Logo"
                width={24}
                height={24}
              />
              <span className="text-base tracking-tight">Lerpz AI</span>
            </Link>
            <p className="text-sm leading-relaxed text-muted-foreground">
              The AI portal built for modern organisations. Smarter tools, fewer
              silos, zero friction.
            </p>

            {/* Social icons */}
            <div className="flex items-center gap-3">
              {socialLinks.map((social) => {
                const Icon = social.icon;
                return (
                  <a
                    key={social.label}
                    href={social.href}
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label={social.label}
                    className="flex size-8 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
                  >
                    <Icon className="size-4" />
                  </a>
                );
              })}
            </div>
          </div>

          {/* Link columns */}
          {Object.entries(footerLinks).map(([group, links]) => (
            <div key={group} className="flex flex-col gap-3">
              <h4 className="text-sm font-semibold text-foreground">{group}</h4>
              <ul className="flex flex-col gap-2">
                {links.map((link) => (
                  <li key={link.href}>
                    <Link
                      href={link.href}
                      className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                    >
                      {link.label}
                    </Link>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>

        {/* Bottom bar */}
        <div className="mt-10 flex flex-col items-center justify-between gap-3 border-t border-border/60 pt-6 text-xs text-muted-foreground sm:flex-row">
          <p>© {year} Lerpz AI. All rights reserved.</p>
          <p className="flex items-center gap-1">
            Made with{" "}
            <MdFavorite
              className="size-3.5 text-destructive"
              aria-label="love"
            />{" "}
            by the Lerpz team
          </p>
        </div>
      </div>
    </footer>
  );
}