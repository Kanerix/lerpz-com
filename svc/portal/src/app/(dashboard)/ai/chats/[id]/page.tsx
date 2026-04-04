"use client";

import { Skeleton } from "@lerpz/ui/components/skeleton";
import { use } from "react";
import type { ChatMessage } from "@/hooks/useChat";
import { useGetChat } from "@/services/api/chats/chats";
import { useAi } from "../../layout";
import ChatView from "../chat-view";

interface ChatPageProps {
  params: Promise<{ id: string }>;
}

export default function ChatPage({ params }: ChatPageProps) {
  const { id } = use(params);
  const { chatMessages, isChatStreaming, chatError, conversationId } = useAi();

  // Use the live in-memory stream only when this exact conversation is active.
  // This covers the case where the user just created a new chat and was
  // redirected here via router.replace — chatMessages is still populated.
  const isLive = conversationId === id;

  const { data: response, isLoading } = useGetChat(id, {
    query: {
      // Skip the network request while the SSE stream is still live; the
      // stream already has the full message history in chatMessages.
      enabled: !isLive,
    },
  });

  if (!isLive && isLoading) {
    return <ChatLoadingSkeleton />;
  }

  if (response?.status !== 200) {
    return (
      <p className="text-muted-foreground px-2 py-4 text-center text-xs">
        Error: {response?.status}
      </p>
    );
  }

  const messages: ChatMessage[] = isLive
    ? chatMessages
    : response.data
      ? response.data.messages.map((m) => ({
          role: m.role as "user" | "assistant",
          content: m.content,
        }))
      : [];

  return (
    <ChatView
      messages={messages}
      isStreaming={isLive && isChatStreaming}
      error={isLive ? chatError : null}
    />
  );
}

function ChatLoadingSkeleton() {
  return (
    <div className="mx-auto max-w-[750px] flex flex-col gap-4 pt-4 pb-8 px-4">
      <div className="flex justify-end gap-3">
        <Skeleton className="h-10 w-56 rounded-2xl rounded-br-md" />
        <Skeleton className="size-7 rounded-full shrink-0 mt-1" />
      </div>
      <div className="flex justify-start gap-3">
        <Skeleton className="size-7 rounded-full shrink-0 mt-1" />
        <Skeleton className="h-24 w-80 rounded-2xl rounded-bl-md" />
      </div>
      <div className="flex justify-end gap-3">
        <Skeleton className="h-10 w-40 rounded-2xl rounded-br-md" />
        <Skeleton className="size-7 rounded-full shrink-0 mt-1" />
      </div>
      <div className="flex justify-start gap-3">
        <Skeleton className="size-7 rounded-full shrink-0 mt-1" />
        <Skeleton className="h-32 w-96 rounded-2xl rounded-bl-md" />
      </div>
    </div>
  );
}
