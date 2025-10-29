"use client";

import { useChat } from "@/hooks/useChat";

export default function ChatBox() {
  const { data, error, isLoading } = useChat();

  if (error) return <div>failed to load</div>;
  if (isLoading) return <div>loading...</div>;
  return <div>{JSON.stringify(data)}</div>;
}
