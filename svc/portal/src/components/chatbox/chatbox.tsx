"use client";

import { Button } from "@lerpz/ui/components/button";
import { Card, CardContent } from "@lerpz/ui/components/card";
import { Textarea } from "@lerpz/ui/components/textarea";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { cn } from "@lerpz/ui/lib/utils";
import { Send, Settings, WandSparkles } from "lucide-react";
import { AnimatePresence, motion } from "motion/react";
import { useState } from "react";
import ChatboxSettingsImage from "./settings-image";

type ChatBoxVariant = "chat" | "image" | "video";
interface ImageChatBoxProps {
  kind: ChatBoxVariant;
  onSubmit: () => void;
}

export default function ImageChatbox({ kind, onSubmit }: ImageChatBoxProps) {
  const [showSettings, setShowSettings] = useState(true);

  const handleSettingsChange = () => {
    setShowSettings((o) => !o);
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.67, ease: "easeOut" }}
    >
      <aside className="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-full max-w-[800px] p-4">
        <Card className="rounded-4xl">
          <CardContent className="flex flex-col gap-y-5">
            <ChatArea isForMobile={true} />
            <TopSection
              onSettingsToggle={handleSettingsChange}
              onSubmit={onSubmit}
            />
            <AnimatePresence initial={false}>
              {kind === "image" && showSettings && (
                <motion.div
                  key="settings-panel"
                  initial={{ opacity: 0, height: 0 }}
                  animate={{ opacity: 1, height: "auto" }}
                  exit={{ opacity: 0, height: 0 }}
                  transition={{ duration: 0.67, ease: "easeInOut" }}
                  style={{ overflow: "hidden" }}
                >
                  <ChatboxSettingsImage />
                </motion.div>
              )}
            </AnimatePresence>
          </CardContent>
        </Card>
      </aside>
    </motion.div>
  );
}

interface ChatAreaProps {
  onSettingsToggle: () => void;
}

function TopSection({
  onSettingsToggle,
  onSubmit,
}: ChatAreaProps & Pick<ImageChatBoxProps, "onSubmit">) {
  return (
    <div className="flex gap-x-4">
      <Tooltip>
        <TooltipTrigger
          render={
            <Button
              className="hidden sm:flex"
              variant="outline"
              size="icon"
              aria-label="Show/hide settings"
              onClick={onSettingsToggle}
            >
              <Settings />
            </Button>
          }
        />
        <TooltipContent>
          <p>Show/hide chat settings</p>
        </TooltipContent>
      </Tooltip>
      <ChatArea isForMobile={false} />
      <Tooltip>
        <TooltipTrigger
          render={
            <Button variant="outline" size="icon" aria-label="Enhance prompt">
              <WandSparkles />
            </Button>
          }
        />
        <TooltipContent>
          <p>Enhance the image prompt!</p>
        </TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger
          render={
            <Button
              className="ml-auto"
              aria-label="Generate image"
              onClick={onSubmit}
            >
              <Send />
              Generate
            </Button>
          }
        />
        <TooltipContent>
          <p>Generate the image</p>
        </TooltipContent>
      </Tooltip>
    </div>
  );
}

function ChatArea({ isForMobile: isMobile }: { isForMobile: boolean }) {
  return (
    <Textarea
      placeholder="Describe your image..."
      className={cn("grow", isMobile ? "sm:block hidden" : "sm:hidden block")}
    />
  );
}
