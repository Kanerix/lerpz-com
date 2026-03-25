"use client";

import { Button } from "@lerpz/ui/components/button";
import { Card, CardContent } from "@lerpz/ui/components/card";
import { Textarea } from "@lerpz/ui/components/textarea";
import { cn } from "@lerpz/ui/lib/utils";
import { ArrowUp, LoaderPinwheel } from "lucide-react";
import { motion } from "motion/react";
import type { ChangeEventHandler } from "react";
import { useEffect, useRef } from "react";
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
        <Card className="rounded-4xl">
          <CardContent className="flex flex-col">
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
  const { isPending, submit, isSubmitPending } = useChatbox();

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
        {isSubmitPending ? (
          <LoaderPinwheel className="animate-spin" />
        ) : (
          <ArrowUp />
        )}
      </Button>
    </div>
  );
}

function PromptInput({ isMobile, className }: PromptInputProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const { mode: variant, isPending } = useChatbox();
  const { prompt, setPrompt } = useChatboxStore();

  const handleChange: ChangeEventHandler<HTMLTextAreaElement> = (e) => {
    setPrompt(e.target.value);
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
      className={cn(
        "grow border-none ring-none outline-none",
        className,
        isMobile ? "block sm:hidden" : "hidden sm:block",
      )}
    />
  );
}
