import type { ReactNode } from "react";
import { Chatbox, ChatboxProvider } from "@/components/chatbox";

interface LayoutProps {
  children: ReactNode;
}

export default function Layout({ children }: LayoutProps) {
  return (
    <ChatboxProvider>
      {children}
      <Chatbox />
    </ChatboxProvider>
  );
}
