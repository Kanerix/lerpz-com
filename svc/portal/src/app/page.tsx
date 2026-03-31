import { Footer, Header, HeroSection, SolutionSection } from "@/components/landing-page";

export default function HomePage() {
  return (
    <div className="relative flex min-h-screen flex-col">
      {/* Page-level glow — sits behind everything, not clipped by any container */}
      <div
        aria-hidden
        className="pointer-events-none fixed inset-0 -z-10 flex items-start justify-center"
      >
        <div className="mt-16 h-[600px] w-[900px] rounded-full bg-primary/10 blur-3xl" />
      </div>

      <Header />
      <main className="mx-auto w-full max-w-[1024px] flex-1 px-4 py-12 md:py-16">
        <HeroSection />
        <SolutionSection />
      </main>
      <Footer />
    </div>
  );
}