"use client";

import Image from "next/image";
import { useAi } from "../layout";

export default function Images() {
  const { generatedImage, isImageLoading, imageError } = useAi();

  return (
    <div className="max-w-[1024px] w-full h-full mx-auto">
      {imageError && (
        <p className="text-sm text-destructive mb-4">{imageError}</p>
      )}
      {generatedImage ? (
        <Image
          src={generatedImage}
          alt="Generated image"
          width={1200}
          height={800}
          unoptimized
        />
      ) : (
        <p>
          {isImageLoading
            ? "Generating image..."
            : "Describe an image in the chatbox below to generate one!"}
        </p>
      )}
    </div>
  );
}