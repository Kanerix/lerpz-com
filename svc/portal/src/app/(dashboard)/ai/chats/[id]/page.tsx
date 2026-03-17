"use client";

import { useAi } from "../../layout";
import ChatView from "../chat-view";

export default function ChatPage() {
  const { chatMessages, isChatStreaming, chatError } = useAi();

  return (
    <ChatView
      messages={chatMessages}
      isStreaming={isChatStreaming}
      error={chatError}
    />
  );
}