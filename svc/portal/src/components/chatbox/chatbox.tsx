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
  Camera,
  ImagePlus,
  LoaderPinwheel,
  Send,
  Settings,
  WandSparkles,
} from "lucide-react";
import { motion } from "motion/react";
import type { ChangeEventHandler } from "react";
import { useEffect, useRef } from "react";
import { toast } from "sonner";
import { useChatboxStore } from "@/store/chatbox.store";
import ImageShelf from "./image-shelf";
import { type ChatboxMode, useChatbox } from "./provider";
import ChatboxSettings from "./settings";

interface ChatareaProps {
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
  const { showSettings, hasPendingWork } = useChatbox();

  useEffect(() => {
    if (!hasPendingWork) return;

    const handleBeforeUnload = (event: BeforeUnloadEvent) => {
      event.preventDefault();
      return "";
    };

    window.addEventListener("beforeunload", handleBeforeUnload);

    return () => {
      window.removeEventListener("beforeunload", handleBeforeUnload);
    };
  }, [hasPendingWork]);

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
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  const {
    mode,
    setShowSettings,
    enhancePrompt,
    isEnhancePending,
    allowImageUploads,
    generateImage,
    isGeneratePending,
    editImage,
    isEditPending,
    hasPendingWork,
  } = useChatbox();

  const { prompt, setPrompt, uploadedImages, addUploadedImages } =
    useChatboxStore();

  const toggleSettings = () => {
    setShowSettings((old) => !old);
  };

  const handleEnhance = async () => {
    if (!prompt) return;
    const newPrompt = await enhancePrompt(prompt);
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

    event.target.value = "";
  };

  const triggerFilePicker = () => {
    fileInputRef.current?.click();
  };

  const handleAddImages = () => {
    if (!allowImageUploads) return;
    triggerFilePicker();
  };

  const handleSubmit = async () => {
    const trimmed = prompt.trim();
    if (!trimmed) return;

    switch (mode) {
      case "image":
        if (uploadedImages.length > 0) {
          await editImage();
        } else {
          await generateImage();
        }
        return;
      case "video":
        // TODO: implement when ready
        toast.error("Video feature is available.");
        return;
      case "chat":
        // TODO: implement when ready
        toast.error("Chat feature is available.");
        return;
    }
  };

  return (
    <div className="grid grid-cols-[max-content_1fr_max-content_max-content] gap-4">

      {/* IMAGES UPLOAD BUTTON */}
      <Tooltip>
        <TooltipTrigger
          render={
            <Button
              className="hidden sm:flex"
              variant="outline"
              size="icon"
              aria-label="Add images"
              disabled={
                !allowImageUploads || isGeneratePending || isEditPending
              }
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
              ? "Add images to your prompt"
              : "Image uploads are disabled"}
          </p>
        </TooltipContent>
      </Tooltip>

      {/* TEXT AREA */}
      <PromptInput className="row-span-2" isMobile={false} />

      {/* GENERATE BUTTON */}
      <Button
        onClick={handleSubmit}
        disabled={hasPendingWork || !prompt?.trim()}
        className="col-span-2 w-full ml-auto sm:ml-0"
        aria-label="Generate image"
      >
        {isGeneratePending || isEditPending ? (
          <LoaderPinwheel className="animate-spin" />
        ) : (
          <Send />
        )}
        Send
      </Button>

      {/* CAMERA BUTTON */}
      <Tooltip>
        <TooltipTrigger
          render={
            <Button
              className="hidden sm:flex"
              variant="outline"
              size="icon"
              aria-label="Take photo"
              disabled={
                !allowImageUploads || isGeneratePending || isEditPending
              }
              onClick={handleAddImages}
            >
              <Camera />
            </Button>
          }
        />
        <TooltipContent>
          <p>Take a photo using camera</p>
        </TooltipContent>
      </Tooltip>

      {/* ENHANCE BUTTON */}
      <Tooltip>
        <TooltipTrigger
          render={
            <Button
              onClick={handleEnhance}
              disabled={hasPendingWork || !prompt.trim()}
              variant="outline"
              size="icon"
              aria-label="Enhance prompt"
            >
              {isEnhancePending ? (
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

      {/* SETTINGS BUTTON */}
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
          <p>Show/hide settings</p>
        </TooltipContent>
      </Tooltip>
    </div>
  );
}

function PromptInput({ isMobile, className }: ChatareaProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const { mode: variant, hasPendingWork } = useChatbox();
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
      disabled={hasPendingWork}
      placeholder={chatareaPlaceholder[variant]}
      value={prompt}
      onChange={handleChange}
      className={cn(
        "grow",
        className,
        isMobile ? "block sm:hidden mb-4" : "hidden sm:block",
      )}
    />
  );
}
