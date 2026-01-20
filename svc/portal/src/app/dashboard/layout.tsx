import type { ReactNode } from "react";
import { AuthGuard } from "@/components/auth-guard";

interface LayoutProps {
  children: ReactNode;
}

export default function ProtectedLayout({ children }: LayoutProps) {
  return <AuthGuard>{children}</AuthGuard>;
}
