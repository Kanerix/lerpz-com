import MsalProvider from "@/components/providers/msal-provider";
import { QueryProvider } from "@/components/providers/query-provider";
import { ThemeProvider } from "@/components/providers/theme-provider";
import "@lerpz/ui/globals.css";
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import { type ReactNode, Suspense } from "react";

const inter = Inter({
  variable: "--font-inter",
  weight: ["100", "200", "300", "400", "500", "600", "700", "800", "900"],
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Lerpz AI",
  description: "An AI portal application for your organization",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head />
      <body className={`${inter.variable} antialiased`}>
        <Suspense fallback="Loading...">
          <MsalProvider>
            <QueryProvider>
              <ThemeProvider
                attribute="class"
                defaultTheme="system"
                enableSystem
                disableTransitionOnChange
              >
                {children}
              </ThemeProvider>
            </QueryProvider>
          </MsalProvider>
        </Suspense>
      </body>
    </html>
  );
}
