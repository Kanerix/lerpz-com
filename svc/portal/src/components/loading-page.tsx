import Image from "next/image";

interface LoadingPageProps {
  message?: string;
}

export function LoadingPage({ message = "Loading..." }: LoadingPageProps) {
  return (
    <div className="flex min-h-screen items-center justify-center">
      <div className="flex flex-col items-center gap-4">
        <Image
          src="/lerpz.svg"
          alt="Lerpz"
          width={48}
          height={48}
          className="h-12 w-12 animate-pulse"
          priority
        />
        <p className="text-sm text-muted-foreground">{message}</p>
      </div>
    </div>
  );
}
