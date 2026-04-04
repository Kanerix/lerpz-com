"use client";

import { Avatar, AvatarFallback } from "@lerpz/ui/components/avatar";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { cn } from "@lerpz/ui/lib/utils";
import { AnimatePresence, motion } from "motion/react";
import { useEffect, useRef } from "react";
import { MdLoop, MdPerson, MdSmartToy } from "react-icons/md";
import type { ChatMessage } from "@/hooks/useChat";

interface ChatViewProps {
  messages: ChatMessage[];
  isStreaming: boolean;
  error: string | null;
}

export default function ChatView({
  messages,
  isStreaming,
  error,
}: ChatViewProps) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, []);

  if (messages.length === 0) {
    return <EmptyState />;
  }

  return (
    <ScrollArea className="h-[calc(100vh-220px)] w-full">
      <div className="mx-auto max-w-[750px] flex flex-col gap-4 pb-8">
        <AnimatePresence initial={false}>
          {messages.map((message, index) => (
            <motion.div
              key={`${message.role}`}
              initial={{ opacity: 0, y: 8 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.2, ease: "easeOut" }}
              className={cn(
                "flex gap-3",
                message.role === "user" ? "justify-end" : "justify-start",
              )}
            >
              {message.role === "assistant" && (
                <Avatar size="sm" className="mt-1 shrink-0">
                  <AvatarFallback>
                    <MdSmartToy className="size-3.5" />
                  </AvatarFallback>
                </Avatar>
              )}

              <div
                className={cn(
                  "max-w-[80%] rounded-2xl px-4 py-2.5 text-sm leading-relaxed whitespace-pre-wrap break-words",
                  message.role === "user"
                    ? "bg-primary text-primary-foreground rounded-br-md"
                    : "bg-muted text-foreground rounded-bl-md",
                )}
              >
                {message.content}
                {message.role === "assistant" &&
                  isStreaming &&
                  index === messages.length - 1 && (
                    <span className="inline-block w-1.5 h-4 ml-0.5 bg-foreground/70 animate-pulse rounded-sm align-text-bottom" />
                  )}
              </div>

              {message.role === "user" && (
                <Avatar size="sm" className="mt-1 shrink-0">
                  <AvatarFallback>
                    <MdPerson className="size-3.5" />
                  </AvatarFallback>
                </Avatar>
              )}
            </motion.div>
          ))}
        </AnimatePresence>

        {isStreaming && messages[messages.length - 1]?.role === "user" && (
          <motion.div
            initial={{ opacity: 0, y: 8 }}
            animate={{ opacity: 1, y: 0 }}
            className="flex gap-3 justify-start"
          >
            <Avatar size="sm" className="mt-1 shrink-0">
              <AvatarFallback>
                <MdSmartToy className="size-3.5" />
              </AvatarFallback>
            </Avatar>
            <div className="bg-muted rounded-2xl rounded-bl-md px-4 py-3">
              <MdLoop className="size-4 animate-spin text-muted-foreground" />
            </div>
          </motion.div>
        )}

        {error && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="mx-auto text-sm text-destructive bg-destructive/10 rounded-lg px-4 py-2"
          >
            {error}
          </motion.div>
        )}

        <div ref={bottomRef} />
      </div>
    </ScrollArea>
  );
}

function EmptyState() {
  return (
    <div className="flex flex-col items-center justify-center h-[calc(100vh-220px)] text-center gap-4">
      <motion.div
        initial={{ opacity: 0, scale: 0.9 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 0.3, ease: "easeOut" }}
        className="flex flex-col items-center gap-3"
      >
        <div className="bg-muted rounded-full p-4">
          <MdSmartToy className="size-8 text-muted-foreground" />
        </div>
        <h2 className="text-xl font-semibold tracking-tight">
          Start a conversation
        </h2>
        <p className="text-sm text-muted-foreground max-w-sm">
          Type a message below to begin chatting. Your conversation will be
          saved automatically.
        </p>
      </motion.div>
    </div>
  );
}
