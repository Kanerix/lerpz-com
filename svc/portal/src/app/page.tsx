import Footer from "@/components/footer";
import Header from "@/components/header";

export default function HomePage() {
  return (
    <>
      <Header />
      <main className="m-auto max-w-[1024px] min-h-screen p-4 mt-8 md:mt-24">
        <HeroSection />
        <SolutionSection />
      </main>
      <Footer />
    </>
  );
}

function HeroSection() {
  return (
    <div className="grid grid-cols-3 gap-4 grid-rows-2 ">
      <div className="col-span-3 xl:col-span-2">
        <h1 className="text-4xl md:text-6xl xl:text-8xl font-bold">
          Unlock the Power of AI for Your Business
        </h1>
        <h3 className="text-2xl md:text-3xl xl:text-4xl font-medium pt-4">
          Access a curated ecosystem of AI tools, resources, and expertise to
          drive innovation and efficiency across your organization.
        </h3>
        <input placeholder="Newsletter..." />
      </div>
      <div>SOME COOL CONTENT</div>
    </div>
  );
}

function SolutionSection() {
  return <div className="h-screen">We got chat, images, and MORE!</div>;
}
