"use client";

import { useEffect } from "react";
import { useAi } from "../layout";
import ChatView from "./chat-view";

export default function Chat() {
    const { chatMessages, isChatStreaming, chatError, resetChat } = useAi();

    // Ensure arriving at the "New Chat" page always starts a fresh conversation,
    // even if the previous page left an active conversationId in the hook.
    useEffect(() => {
        resetChat();
    }, [resetChat]);

    return (
        <ChatView
            messages={chatMessages}
            isStreaming={isChatStreaming}
            error={chatError}
        />
    );
}
