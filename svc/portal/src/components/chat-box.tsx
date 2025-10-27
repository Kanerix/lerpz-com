"use client";

import { useChat } from "@/hooks/useChat";

export default function ChatBox() {
  const { data } = useChat();

  return <div>
    {JSON.stringify(data)}
  </div>;
}
