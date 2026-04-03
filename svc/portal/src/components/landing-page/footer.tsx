import Image from "next/image";
import Link from "next/link";
import { MdFavorite } from "react-icons/md";
import { SiGithub } from "react-icons/si";

const footerLinks = {
  Product: [
    { label: "Changelog", href: "/changelog" },
    { label: "Roadmap", href: "/roadmap" },
    { label: "Status", href: "/status" },
  ],
  Resources: [
    { label: "Documentation", href: "/docs" },
    { label: "API Reference", href: "/docs/api" },
    { label: "Support", href: "/support" },
  ],
  Company: [
    { label: "About", href: "/about" },
    { label: "Contact", href: "/contact" },
  ],
  Legal: [
    { label: "Privacy Policy", href: "/legal/privacy" },
    { label: "Terms of Service", href: "/legal/terms" },
    { label: "Cookie Policy", href: "/legal/cookies" },
    { label: "Security", href: "/legal/security" },
  ],
};

export default function Footer() {
  return (
    <footer className="border-t border-border/60 bg-card/40">
      <div className="mx-auto max-w-[1024px] px-4 py-12">
        {/* Top row: brand + columns */}
        <div className="grid grid-cols-2 gap-10 md:grid-cols-5">
          {/* Brand column */}
          <div className="col-span-2 flex flex-col gap-4 md:col-span-1">
            <Link
              href="/"
              className="flex items-center gap-2 font-semibold text-foreground transition-opacity hover:opacity-75"
            >
              <Image src="/lerpz.svg" alt="Lerpz Logo" width={22} height={22} />
              <span className="text-sm tracking-tight">Lerpz AI</span>
            </Link>
            <p className="text-sm leading-relaxed text-muted-foreground">
              The AI portal for your organisation. Maintained centrally so
              everyone works from the same tooling.
            </p>
            <a
              href="https://github.com/lerpz"
              target="_blank"
              rel="noopener noreferrer"
              aria-label="GitHub"
              className="w-fit text-muted-foreground transition-colors hover:text-foreground"
            >
              <SiGithub className="size-4" />
            </a>
          </div>

          {/* Link columns */}
          {Object.entries(footerLinks).map(([group, links]) => (
            <div key={group} className="flex flex-col gap-3">
              <h4 className="text-xs font-semibold uppercase tracking-wider text-foreground">
                {group}
              </h4>
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
          <p>© 2026 Lerpz AI. All rights reserved.</p>
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
