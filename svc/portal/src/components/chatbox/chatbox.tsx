"use client";

import { Button } from "@lerpz/ui/components/button";
import { Card, CardContent } from "@lerpz/ui/components/card";
import { Textarea } from "@lerpz/ui/components/textarea";
import { cn } from "@lerpz/ui/lib/utils";
import { MdArrowUpward, MdLoop } from "react-icons/md";
import { motion } from "motion/react";
import type { ChangeEventHandler, KeyboardEventHandler } from "react";
import { useCallback, useEffect, useRef } from "react";
import ImageShelf from "./image-shelf";
import { type ChatboxMode, useChatbox } from "./provider";
import ChatboxSettings from "./settings";
import { useChatboxStore } from "./store";

interface PromptInputProps {
  isMobile: boolean;
  className?: string;
}

const settingsVariants = {
  visible: {
    opacity: 1,
    height: "auto",
    transition: { duration: 0.2, ease: "easeOut" },
  },
  hidden: {
    opacity: 0,
    height: 0,
    transition: { duration: 0.2, ease: "easeIn" },
  },
} as const;

const chatareaPlaceholder: Record<ChatboxMode, string> = {
  chat: "Send a message!",
  image: "Describe your image!",
  video: "Describe your video!",
};

export default function Chatbox() {
  const { isPending, showSettings } = useChatbox();
  const { setChatboxAnchor } = useChatboxStore();

  const cardRef = useCallback(
    (node: HTMLDivElement | null) => {
      setChatboxAnchor(node);
    },
    [setChatboxAnchor],
  );

  useEffect(() => {
    if (!isPending) return;

    const handleBeforeUnload = (event: BeforeUnloadEvent) => {
      event.preventDefault();
      return "";
    };

    window.addEventListener("beforeunload", handleBeforeUnload);

    return () => {
      window.removeEventListener("beforeunload", handleBeforeUnload);
    };
  }, [isPending]);

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.25, ease: "easeIn" }}
    >
      <aside className="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-full max-w-[750px] p-4">
        <ImageShelf />
        <Card ref={cardRef} className="rounded-4xl py-3 gap-3">
          <CardContent className="flex flex-col px-3">
            <PromptInput isMobile={true} />
            <ChatboxToolbar />
            <motion.div
              variants={settingsVariants}
              animate={showSettings ? "visible" : "hidden"}
              transition={{ duration: 0.67, ease: "easeOut" }}
            >
              <ChatboxSettings />
            </motion.div>
          </CardContent>
        </Card>
      </aside>
    </motion.div>
  );
}

function ChatboxToolbar() {
  const { isPending, submit } = useChatbox();
  const { prompt } = useChatboxStore();

  return (
    <div className="flex gap-4">
      {/* TEXT AREA */}
      <PromptInput className="grow" isMobile={false} />

      {/* GENERATE BUTTON */}
      <Button
        size="icon"
        onClick={submit}
        disabled={isPending || !prompt?.trim()}
        aria-label="Send prompt"
      >
        {isPending ? <MdLoop className="animate-spin" /> : <MdArrowUpward />}
      </Button>
    </div>
  );
}

function PromptInput({ isMobile, className }: PromptInputProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const { mode: variant, isPending, submit } = useChatbox();
  const { prompt, setPrompt } = useChatboxStore();

  const handleChange: ChangeEventHandler<HTMLTextAreaElement> = (e) => {
    setPrompt(e.target.value);
  };

  const handleKeyDown: KeyboardEventHandler<HTMLTextAreaElement> = (e) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      if (!isPending && prompt?.trim()) {
        submit();
      }
    }
  };

  useEffect(() => {
    textareaRef.current?.focus();
  }, []);

  return (
    <Textarea
      ref={textareaRef}
      disabled={isPending}
      placeholder={chatareaPlaceholder[variant]}
      value={prompt}
      onChange={handleChange}
      onKeyDown={handleKeyDown}
      rows={1}
      className={cn(
        "grow min-h-0 border-none shadow-none ring-0 outline-none focus-visible:ring-0 focus-visible:border-none bg-transparent",
        className,
        isMobile ? "block sm:hidden" : "hidden sm:block",
      )}
    />
  );
}
