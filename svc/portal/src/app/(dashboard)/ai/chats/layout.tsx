"use client";

import { useEffect } from "react";
import { toast } from "sonner";
import { useChatbox } from "@/components/chatbox/provider";

export default function Layout({ children }: { children: React.ReactNode }) {
  const { setMode } = useChatbox();

  useEffect(() => {
    toast("Event has been created.");
  }, []);

  useEffect(() => {
    setMode("chat");
  }, [setMode]);

  return children;
}
