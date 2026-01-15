import { Toaster } from "@lerpz/ui/components/sonner";
import { SWRProvider } from "@/components/swr-provider";
import { ThemeProvider } from "@/components/theme-provider";
import "@lerpz/ui/globals.css";
import type { Metadata } from "next";
import { Poppins } from "next/font/google";
import { type ReactNode, Suspense } from "react";

const poppins = Poppins({
  variable: "--font-poppins",
  weight: ["100", "200", "300", "400", "500", "600", "700", "800", "900"],
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Lerpz AI",
  description: "An AI chat and image application",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: ReactNode;
}>) {
  return (
    <html suppressHydrationWarning lang="en">
      <head />
      <body className={`${poppins.variable} antialiased`}>
        <Suspense fallback="Loading...">
          <SWRProvider>
            <ThemeProvider
              attribute="class"
              defaultTheme="system"
              enableSystem
              disableTransitionOnChange
            >
              {children}
            </ThemeProvider>
            <Toaster />
          </SWRProvider>
        </Suspense>
      </body>
    </html>
  );
}
