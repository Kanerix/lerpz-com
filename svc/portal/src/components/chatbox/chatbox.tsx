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
import { motion } from "motion/react";
import { useState } from "react";
import ChatboxSettingsImage from "./settings-image";

type ChatboxVariant = "chat" | "image" | "video";

interface ChatboxProps {
  variant: ChatboxVariant;
}

interface ChatToolbarProps extends Pick<ChatboxProps, "variant"> {
  onSettingsToggle: () => void;
}

interface ChatareaProps extends Pick<ChatboxProps, "variant"> {
  isMobile: boolean;
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

const submitButtonPlaceholder: Record<ChatboxVariant, string> = {
  chat: "Send",
  image: "Generate",
  video: "Generate",
};

const chatareaPlaceholder: Record<ChatboxVariant, string> = {
  chat: "Send a message!",
  image: "Describe your image!",
  video: "Describe your video!",
};

export default function Chatbox({ variant }: ChatboxProps) {
  const [showSettings, setShowSettings] = useState(true);

  const handleSettingsChange = () => {
    setShowSettings((o) => !o);
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.25, ease: "easeIn" }}
    >
      <aside className="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-full max-w-[800px] p-4">
        <Card className="rounded-4xl">
          <CardContent className="flex flex-col">
            <Chatarea variant={variant} isMobile={true} />
            <ChatToolbar
              variant={variant}
              onSettingsToggle={handleSettingsChange}
            />
            <motion.div
              variants={settingsVariants}
              animate={showSettings ? "visible" : "hidden"}
              transition={{ duration: 0.67, ease: "easeOut" }}
            >
              {variant === "image" && <ChatboxSettingsImage />}
            </motion.div>
          </CardContent>
        </Card>
      </aside>
    </motion.div>
  );
}

function ChatToolbar({ onSettingsToggle, variant }: ChatToolbarProps) {
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
      <Chatarea isMobile={false} variant={variant} />
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
            <Button className="ml-auto sm:ml-0" aria-label="Generate image">
              <Send />
              {submitButtonPlaceholder[variant]}
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

function Chatarea({ isMobile, variant }: ChatareaProps) {
  return (
    <Textarea
      placeholder={chatareaPlaceholder[variant]}
      className={cn("grow", isMobile ? "block sm:hidden" : "hidden sm:block")}
    />
  );
}
