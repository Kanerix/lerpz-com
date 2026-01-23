"use client";

import { X } from "lucide-react";
import { AnimatePresence, motion } from "motion/react";
import Image from "next/image";
import { useChatboxStore } from "@/store/chatbox.store";

export default function ImageShelf() {
  const { uploadedImages, removeUploadedImage } = useChatboxStore();

  return (
    <div className="flex justify-center gap-x-1 mb-2">
      <AnimatePresence initial={false}>
        {uploadedImages.map((image) => (
          <motion.div
            key={image.id}
            className="group relative h-16 w-16 overflow-hidden rounded-xl border bg-background shadow-sm"
            initial={{ opacity: 0, scale: 0, y: 4 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.9, y: 4 }}
            transition={{ duration: 0.2, ease: "easeOut" }}
          >
            <button
              type="button"
              onClick={() => removeUploadedImage(image.id)}
              className="absolute right-1 top-1 z-10 h-6 w-6
              flex items-center justify-center
              rounded-full bg-background/80 text-foreground shadow-sm
              opacity-0 ring-1 ring-border transition-opacity
              group-hover:opacity-100"
              aria-label={`Remove ${image.file.name || "image"}`}
            >
              <X className="h-3 w-3" />
            </button>

            <Image
              src={image.previewUrl}
              alt={image.file.name || "Uploaded image"}
              fill
              sizes="64px"
              className="object-cover"
            />
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}
