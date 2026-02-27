import { Banner } from "nextra/components";
import { getPageMap } from "nextra/page-map";
import { Footer, Layout, Navbar } from "nextra-theme-docs";
import type { ReactNode } from "react";

export const metadata = {
  title: {
    default: "Lerpz AI Docs",
    template: "%s - Lerpz AI Docs",
  },
  description: "Documentation for Lerpz AI",
};

export default async function DocsLayout({
  children,
}: {
  children: ReactNode;
}) {
  const pageMap = await getPageMap("/docs");

  return (
    <Layout
      banner={<Banner storageKey="lerpz-docs-banner">Lerpz AI Docs</Banner>}
      navbar={<Navbar logo={<span className="font-bold">Lerpz AI</span>} />}
      footer={<Footer>MIT 2025 © Lerpz AI.</Footer>}
      pageMap={pageMap}
    >
      {children}
    </Layout>
  );
}
