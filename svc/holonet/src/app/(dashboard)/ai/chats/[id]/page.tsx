"use client";

import { Skeleton } from "@lerpz/ui/components/skeleton";
import { use, useEffect } from "react";

import { useGetChat } from "@/services/api/chats/chats";
import { useAi } from "../../layout";
import ChatView from "../chat-view";

interface ChatPageProps {
    params: Promise<{ id: string }>;
}

export default function ChatPage({ params }: ChatPageProps) {
    const { id } = use(params);
    const {
        chatMessages,
        isChatStreaming,
        chatError,
        conversationId,
        enterConversation,
    } = useAi();

    // "Live" means the hook is actively holding data for this specific
    // conversation — either because we just created it (and were redirected
    // here) or because the user already sent a message during this session.
    const isLive = conversationId === id && chatMessages.length > 0;

    const { data: response, isLoading } = useGetChat(id, {
        query: {
            // Skip fetching while the SSE stream already has the full history.
            enabled: !isLive,
        },
    });

    // Once the API returns data and we are not yet in "live" mode for this
    // conversation, seed the hook so that the next send() continues it.
    useEffect(() => {
        if (isLive || conversationId === id) return;
        if (response?.status !== 200 || !response.data) return;

        enterConversation(id, response.data.messages);
    }, [id, isLive, conversationId, response, enterConversation]);

    if (!isLive && isLoading) {
        return <ChatLoadingSkeleton />;
    }

    if (!isLive && response?.status !== 200) {
        return (
            <p className="text-muted-foreground px-2 py-4 text-center text-xs">
                Error: {response?.status}
            </p>
        );
    }

    const messages = isLive
        ? chatMessages
        : response?.status === 200 && response.data
          ? response.data.messages
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
