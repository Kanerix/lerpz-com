import Link from "next/link";
import Image from "next/image";
import { Button } from "@lerpz/ui/components/button";

export default function NotFound() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-background px-4">
      <div className="flex flex-col items-center gap-6 text-center max-w-md">
        <Image
          src="/lerpz.svg"
          alt="Lerpz"
          width={48}
          height={48}
          className="h-12 w-12 opacity-40"
          priority
        />

        <div className="flex flex-col items-center gap-2">
          <h1 className="text-7xl font-bold tracking-tighter text-foreground">
            404
          </h1>
          <h2 className="text-xl font-semibold tracking-tight text-foreground">
            Page not found
          </h2>
          <p className="text-sm text-muted-foreground">
            The page you&apos;re looking for doesn&apos;t exist or has been
            moved.
          </p>
        </div>

        <div className="flex items-center gap-3">
          <Button variant="default" size="lg" render={<Link href="/" />}>
            Go home
          </Button>
          <Button variant="outline" size="lg" render={<Link href="/login" />}>
            Sign in
          </Button>
        </div>
      </div>
    </div>
  );
}