"use client";

import { useImageSse } from "@/hooks/useImage";
import { Button } from "@lerpz/ui/components/button";
import Image from 'next/image'

export default function Images() {
  const { image: imageData, start } = useImageSse();

  const handleClick = () => {
    start("Show me a playful Toller dog!");
  };

  return <div className="max-w-[1024px] w-full h-full mx-auto">
    {imageData ? (
      <Image
        src={imageData}
        alt="something"
        width={1200}
        height={800}
        unoptimized
      />
    ) : (
      <p>Generate an image (data-string)!</p>
    )}
    <Button onClick={handleClick}>Generate</Button>
  </div>;
}
