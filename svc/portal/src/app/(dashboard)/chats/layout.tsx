import { useEffect } from "react";
import { useChatbox } from "@/components/chatbox/provider";

export default function Layout({ children }: { children: React.ReactNode }) {
  const { setMode } = useChatbox();

  useEffect(() => {
    setMode("chat");
  }, [setMode]);

  return children;
}
