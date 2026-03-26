"use client";

import { Button } from "@lerpz/ui/components/button";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@lerpz/ui/components/select";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import {
  Brain,
  Camera,
  ImagePlus,
  LoaderPinwheel,
  WandSparkles,
} from "lucide-react";
import { type ChangeEventHandler, useCallback, useEffect, useRef } from "react";
import { toast } from "sonner";
import { useChatbox } from "./provider";
import { useChatboxStore } from "./store";

type ModelSelectValue = string | null;

interface ModelSelectItem {
  label: string;
  value: ModelSelectValue;
}

export default function ChatboxSettings() {
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  const {
    mode,
    isPending,
    enhance,
    isEnhancePending,
    allowImageUploads,
    models,
    isModelsLoading,
    loadModels,
  } = useChatbox();
  const { model, setModel, prompt, setPrompt, addUploadedImages } =
    useChatboxStore();

  const modelItems: ModelSelectItem[] =
    models?.map((m) => ({
      label: m.label,
      value: m.value,
    })) ?? [];

  useEffect(() => {
    loadModels(mode);
  }, [mode, loadModels]);

  const handleEnhance = async () => {
    if (!prompt || !enhance) return;
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

    event.target.value = "";
  };

  const triggerFilePicker = () => {
    fileInputRef.current?.click();
  };

  const handleAddImages = () => {
    if (!allowImageUploads) return;
    triggerFilePicker();
  };

  const handleCameraCapture = useCallback(async () => {
    if (!allowImageUploads) return;

    let stream: MediaStream | null = null;

    try {
      stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: "environment" },
      });
    } catch (err) {
      const message =
        err instanceof DOMException && err.name === "NotAllowedError"
          ? "Camera access was denied. Please allow camera permissions and try again."
          : "Could not access the camera. Make sure your device has a camera available.";

      toast.error("Camera error", {
        position: "top-center",
        description: message,
      });
      return;
    }

    try {
      const video = document.createElement("video");
      video.srcObject = stream;
      video.setAttribute("playsinline", "true");
      await video.play();

      await new Promise((resolve) => setTimeout(resolve, 300));

      const canvas = document.createElement("canvas");
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;

      const ctx = canvas.getContext("2d");
      if (!ctx) {
        toast.error("Camera error", {
          position: "top-center",
          description: "Was unable to create a canvas context for the photo.",
        });
        return;
      }

      ctx.drawImage(video, 0, 0);

      const blob = await new Promise<Blob | null>((resolve) =>
        canvas.toBlob(resolve, "image/png"),
      );

      if (!blob) {
        toast.error("Camera error", {
          position: "top-center",
          description: "Failed to capture the photo. Please try again.",
        });
        return;
      }

      const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
      const file = new File([blob], `camera-${timestamp}.png`, {
        type: "image/png",
      });

      addUploadedImages([file]);
    } catch (_) {
      toast.error("Camera error", {
        position: "top-center",
        description: "Something went wrong while taking the photo.",
      });
    } finally {
      stream.getTracks().forEach((track) => {
        track.stop();
      });
    }
  }, [allowImageUploads, addUploadedImages]);

  return (
    <div className="flex mt-4">
      <Select items={modelItems} value={model} onValueChange={setModel}>
        <Tooltip>
          <TooltipTrigger
            render={
              <SelectTrigger className="relative" disabled={isModelsLoading}>
                <div className="flex items-center">
                  {isModelsLoading ? (
                    <LoaderPinwheel className="mr-2 animate-spin" />
                  ) : (
                    <Brain className="mr-2" />
                  )}
                  <SelectValue placeholder="Image model" />
                </div>
              </SelectTrigger>
            }
          />
          <TooltipContent>
            <p>Change image model</p>
          </TooltipContent>
        </Tooltip>
        <SelectContent className="w-fit" alignItemWithTrigger={false}>
          <SelectGroup>
            <SelectLabel>Model</SelectLabel>
            {modelItems.map((m) => (
              <SelectItem key={m.value ?? "default"} value={m.value}>
                {m.label}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>

      <div className="flex gap-x-4 ml-auto">
        {/* CAMERA BUTTON */}
        <Tooltip>
          <TooltipTrigger
            render={
              <Button
                className="hidden sm:flex"
                variant="outline"
                size="icon"
                aria-label="Take photo"
                disabled={!allowImageUploads || isPending}
                onClick={handleCameraCapture}
              >
                <Camera />
              </Button>
            }
          />
          <TooltipContent>
            <p>Take a photo using camera</p>
          </TooltipContent>
        </Tooltip>

        {/* IMAGES UPLOAD BUTTON */}
        <Tooltip>
          <TooltipTrigger
            render={
              <Button
                className="hidden sm:flex"
                variant="outline"
                size="icon"
                aria-label="Add images"
                disabled={!allowImageUploads || isPending}
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

        {/* ENHANCE BUTTON */}
        <Tooltip>
          <TooltipTrigger
            render={
              <Button
                onClick={handleEnhance}
                disabled={isPending || !prompt.trim() || !enhance}
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
      </div>
    </div>
  );
}
