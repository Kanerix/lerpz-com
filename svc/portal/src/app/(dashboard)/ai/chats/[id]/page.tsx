"use client";

import { use } from "react";
import { useAi } from "../../layout";
import ChatView from "../chat-view";

interface ChatPageProps {
  params: Promise<{ id: string }>;
}

export default function ChatPage({ params }: ChatPageProps) {
  const { id: _chatId } = use(params);
  const { chatMessages, isChatStreaming, chatError } = useAi();

  return (
    <ChatView
      messages={chatMessages}
      isStreaming={isChatStreaming}
      error={chatError}
    />
  );
}
