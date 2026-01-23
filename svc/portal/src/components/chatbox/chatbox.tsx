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
import {
  ImagePlus,
  LoaderPinwheel,
  Send,
  Settings,
  WandSparkles,
} from "lucide-react";
import { motion } from "motion/react";
import type { ChangeEventHandler } from "react";
import { useRef } from "react";
import { useChatboxStore } from "@/store/chatbox.store";
import ImageShelf from "./image-shelf";
import { useChatbox } from "./provider";
import ChatboxSettings from "./settings";

type ChatboxVariant = "chat" | "image" | "video";

interface ChatareaProps {
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

export default function Chatbox() {
  const { variant, showSettings } = useChatbox();

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.25, ease: "easeIn" }}
    >
      <aside className="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-full max-w-[850px] p-4">
        {(variant === "image" || variant === "chat") && <ImageShelf />}
        <Card className="rounded-4xl">
          <CardContent className="flex flex-col">
            <Chatarea isMobile={true} />
            <ChatToolbar />
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

function ChatToolbar() {
  const {
    variant,
    setShowSettings,
    enhance,
    isEnhanceLoading,
    allowImageUploads,
  } = useChatbox();
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  const { prompt, setPrompt, addUploadedImages } = useChatboxStore();

  const toggleSettings = () => {
    setShowSettings((old) => !old);
  };

  const handleEnhance = async () => {
    if (!prompt) return; // THROW ERROR
    const newPrompt = await enhance(prompt);
    setPrompt(newPrompt);
  };

  const handleImagesFileSelection: ChangeEventHandler<HTMLInputElement> = (
    event,
  ) => {
    const files = event.target.files;
    if (!files || files.length === 0) return;

    const imageFiles = Array.from(files).filter((file) =>
      file.type.startsWith("image/"),
    );
    if (!imageFiles.length) return;

    addUploadedImages(imageFiles);

    // clear the input so selecting the same file again will still trigger onChange
    event.target.value = "";
  };

  const triggerFilePicker = () => {
    fileInputRef.current?.click();
  };

  const handleAddImages = () => {
    if (!allowImageUploads) return;
    triggerFilePicker();
  };

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
              onClick={toggleSettings}
            >
              <Settings />
            </Button>
          }
        />
        <TooltipContent>
          <p>Show/hide chat settings</p>
        </TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger
          render={
            <Button
              className="hidden sm:flex"
              variant="outline"
              size="icon"
              aria-label="Add images"
              disabled={!allowImageUploads}
              onClick={handleAddImages}
            >
              <input
                ref={fileInputRef}
                type="file"
                accept="image/*"
                multiple
                className="hidden"
                onChange={handleImagesFileSelection}
              />
              <ImagePlus />
            </Button>
          }
        />
        <TooltipContent>
          <p>
            {allowImageUploads
              ? "Add images to prompt"
              : "Image uploads are disabled"}
          </p>
        </TooltipContent>
      </Tooltip>
      <Chatarea isMobile={false} />
      <Tooltip>
        <TooltipTrigger
          render={
            <Button
              onClick={handleEnhance}
              disabled={isEnhanceLoading}
              variant="outline"
              size="icon"
              aria-label="Enhance prompt"
            >
              {isEnhanceLoading ? (
                <LoaderPinwheel className="animate-spin" />
              ) : (
                <WandSparkles />
              )}
            </Button>
          }
        />
        <TooltipContent>
          <p>Enhance the image prompt!</p>
        </TooltipContent>
      </Tooltip>
      <Button className="ml-auto sm:ml-0" aria-label="Generate image">
        <Send />
        {submitButtonPlaceholder[variant]}
      </Button>
    </div>
  );
}

function Chatarea({ isMobile }: ChatareaProps) {
  const { variant, isEnhanceLoading } = useChatbox();
  const { prompt, setPrompt } = useChatboxStore();

  const handleChange: ChangeEventHandler<HTMLTextAreaElement> = (e) => {
    setPrompt(e.target.value);
  };

  return (
    <Textarea
      disabled={isEnhanceLoading}
      placeholder={chatareaPlaceholder[variant]}
      value={prompt || undefined}
      onChange={handleChange}
      className={cn(
        "grow",
        isMobile ? "block sm:hidden mb-4" : "hidden sm:block",
      )}
    />
  );
}
