"use client";

import { Button } from "@lerpz/ui/components/button";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { type ChangeEventHandler, useCallback, useRef } from "react";
import {
    MdAddPhotoAlternate,
    MdAutoFixHigh,
    MdCameraAlt,
    MdLoop,
} from "react-icons/md";
import { toast } from "sonner";
import { ModelSelectorTrigger } from "./model-selector";
import { useChatbox } from "./provider";
import { useChatboxStore } from "./store";

export default function ChatboxSettings() {
    const fileInputRef = useRef<HTMLInputElement | null>(null);

    const { isPending, enhance, isEnhancePending, allowImageUploads } =
        useChatbox();
    const { prompt, setPrompt, addUploadedImages } = useChatboxStore();

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
                    description:
                        "Was unable to create a canvas context for the photo.",
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
                    description:
                        "Failed to capture the photo. Please try again.",
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
            <ModelSelectorTrigger />

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
                                <MdCameraAlt />
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
                                <MdAddPhotoAlternate />
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
                                disabled={
                                    isPending || !prompt.trim() || !enhance
                                }
                                variant="outline"
                                size="icon"
                                aria-label="Enhance prompt"
                            >
                                {isEnhancePending ? (
                                    <MdLoop className="animate-spin" />
                                ) : (
                                    <MdAutoFixHigh />
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
