import type { IconType } from "react-icons";
import {
  MdInsertDriveFile,
  MdCollections,
  MdPhotoSizeSelectLarge,
  MdCropSquare,
} from "react-icons/md";
import { useCallback, useState } from "react";

export type Model = {
  label: string;
  value: string | null;
  description: string;
  modalities: string[];
  features: string[];
  settings: ModelSetting[];
};

export type ModelSetting = {
  key: string;
  name: string;
  icon: IconType;
  tooltip: string;
  defaultValue: string;
  values: {
    value: string | number;
    label: string;
  }[];
};

export type ChatboxVariant = "chat" | "image" | "video";

const fakeDelay = (ms: number) =>
  new Promise<void>((resolve) => setTimeout(resolve, ms));

const currentModels: Model[] = [
  {
    label: "GPT Image 1",
    value: "gpt-image-1",
    description:
      "OpenAI's flagship image generation model with support for both generating \
      and editing images. Offers high-quality output with configurable \
      resolution, quality, and background options.",
    modalities: ["image"],
    features: ["generate", "edit"],
    settings: [
      {
        name: "Quality",
        key: "quality",
        icon: MdPhotoSizeSelectLarge,
        tooltip: "Quality of the image",
        defaultValue: "medium",
        values: [
          { value: "low", label: "Low" },
          { value: "medium", label: "Medium" },
          { value: "high", label: "High" },
        ],
      },
      {
        name: "Dimensions",
        key: "size",
        icon: MdCropSquare,
        tooltip: "Dimensions of the image",
        defaultValue: "1024x1024",
        values: [
          { value: "1024x1024", label: "1024x1024" },
          { value: "1536x1024", label: "1536x1024" },
          { value: "1024x1536", label: "1024x1536" },
        ],
      },
      {
        name: "Amount",
        key: "n",
        icon: MdCollections,
        tooltip: "Amount of images",
        defaultValue: "1",
        values: [
          { value: 1, label: "1" },
          { value: 2, label: "2" },
          { value: 3, label: "3" },
          { value: 4, label: "4" },
          { value: 5, label: "5" },
          { value: 6, label: "6" },
        ],
      },
      {
        name: "Filetype",
        key: "output_format",
        icon: MdInsertDriveFile,
        tooltip: "Type of image",
        defaultValue: "png",
        values: [
          { value: "png", label: "PNG" },
          { value: "jpeg", label: "JPG" },
        ],
      },
      {
        name: "Background",
        key: "background",
        icon: MdInsertDriveFile,
        tooltip: "Background of the image",
        defaultValue: "auto",
        values: [
          { value: "auto", label: "Auto" },
          { value: "transparent", label: "Transparent" },
          { value: "opaque", label: "Opaque" },
        ],
      },
    ],
  },
  {
    label: "GPT Image 1.5",
    value: "gpt-image-1.5",
    description:
      "The latest iteration of OpenAI's image generation model with improved \
      quality and WebP output support. Optimized for generation tasks with \
      enhanced detail and coherence.",
    modalities: ["image"],
    features: ["generate", "edit"],
    settings: [
      {
        name: "Quality",
        key: "quality",
        icon: MdPhotoSizeSelectLarge,
        tooltip: "Quality of the image",
        defaultValue: "medium",
        values: [
          { value: "low", label: "Low" },
          { value: "medium", label: "Medium" },
          { value: "high", label: "High" },
        ],
      },
      {
        name: "Dimensions",
        key: "size",
        icon: MdCropSquare,
        tooltip: "Dimensions of the image",
        defaultValue: "1024x1024",
        values: [
          { value: "1024x1024", label: "1024x1024" },
          { value: "1536x1024", label: "1536x1024" },
          { value: "1024x1536", label: "1024x1536" },
        ],
      },
      {
        name: "Amount",
        key: "n",
        icon: MdCollections,
        tooltip: "Amount of image",
        defaultValue: "1",
        values: [
          { value: 1, label: "1" },
          { value: 2, label: "2" },
          { value: 3, label: "3" },
          { value: 4, label: "4" },
          { value: 5, label: "5" },
          { value: 6, label: "6" },
        ],
      },
      {
        name: "Filetype",
        key: "output_format",
        icon: MdInsertDriveFile,
        tooltip: "Type of image",
        defaultValue: "png",
        values: [
          { value: "png", label: "PNG" },
          { value: "jpeg", label: "JPG" },
        ],
      },
      {
        name: "Background",
        key: "background",
        icon: MdInsertDriveFile,
        tooltip: "Background of the image",
        defaultValue: "auto",
        values: [
          { value: "auto", label: "Auto" },
          { value: "transparent", label: "Transparent" },
          { value: "opaque", label: "Opaque" },
        ],
      },
    ],
  },
  {
    label: "GPT Image 1 Mini",
    value: "gpt-image-1-mini",
    description:
      "A lightweight, cost-efficient variant of GPT Image 1 designed for faster \
      generation. Ideal for quick iterations and drafts while still supporting \
      multiple quality levels and formats.",
    modalities: ["image"],
    features: ["generate"],
    settings: [
      {
        name: "Quality",
        key: "quality",
        icon: MdPhotoSizeSelectLarge,
        tooltip: "Quality of the image",
        defaultValue: "medium",
        values: [
          { value: "low", label: "Low" },
          { value: "medium", label: "Medium" },
          { value: "high", label: "High" },
        ],
      },
      {
        name: "Dimensions",
        key: "size",
        icon: MdCropSquare,
        tooltip: "Dimensions of the image",
        defaultValue: "1024x1024",
        values: [
          { value: "1024x1024", label: "1024x1024" },
          { value: "1536x1024", label: "1536x1024" },
          { value: "1024x1536", label: "1024x1536" },
        ],
      },
      {
        name: "Amount",
        key: "n",
        icon: MdCollections,
        tooltip: "Amount of images",
        defaultValue: "1",
        values: [
          { value: 1, label: "1" },
          { value: 2, label: "2" },
          { value: 3, label: "3" },
          { value: 4, label: "4" },
          { value: 5, label: "5" },
          { value: 6, label: "6" },
        ],
      },
      {
        name: "Filetype",
        key: "output_format",
        icon: MdInsertDriveFile,
        tooltip: "Type of image",
        defaultValue: "png",
        values: [
          { value: "png", label: "PNG" },
          { value: "jpeg", label: "JPG" },
        ],
      },
      {
        name: "Background",
        key: "background",
        icon: MdInsertDriveFile,
        tooltip: "Background of the image",
        defaultValue: "auto",
        values: [
          { value: "auto", label: "Auto" },
          { value: "transparent", label: "Transparent" },
          { value: "opaque", label: "Opaque" },
        ],
      },
    ],
  },
  {
    label: "Nano Banana",
    value: "gemini-2.5-flash-image",
    description:
      "Powered by Gemini 2.5 Flash, this model offers fast image generation with \
      flexible aspect ratio options. Great for quick, versatile image creation \
      across a wide range of dimensions.",
    modalities: ["image"],
    features: ["generate"],
    settings: [
      {
        name: "Aspect Ratio",
        key: "aspect_ratio",
        icon: MdCropSquare,
        tooltip: "Aspect ratio of image",
        defaultValue: "1:1",
        values: [
          { value: "1:1", label: "1:1" },
          { value: "3:2", label: "3:2" },
          { value: "2:3", label: "2:3" },
          { value: "3:4", label: "3:4" },
          { value: "4:3", label: "4:3" },
          { value: "4:5", label: "4:5" },
          { value: "5:4", label: "5:4" },
          { value: "9:16", label: "9:16" },
          { value: "16:9", label: "16:9" },
          { value: "21:9", label: "21:9" },
        ],
      },
    ],
  },
  {
    label: "Nano Banana 2",
    value: "gemini-3.1-flash-image",
    description:
      "Powered by Gemini 3.1 Flash, this model delivers fast, high-quality image \
      generation with resolution options up to 4K and an extended range of \
      aspect ratios.",
    modalities: ["image"],
    features: ["generate"],
    settings: [
      {
        name: "Resolution",
        key: "resolution",
        icon: MdPhotoSizeSelectLarge,
        tooltip: "Resolution of the image",
        defaultValue: "1K",
        values: [
          { value: "1K", label: "1K" },
          { value: "2K", label: "2K" },
          { value: "4K", label: "4K" },
        ],
      },
      {
        name: "Aspect Ratio",
        key: "aspect_ratio",
        icon: MdCropSquare,
        tooltip: "Aspect ratio of image",
        defaultValue: "1:1",
        values: [
          { value: "1:1", label: "1:1" },
          { value: "3:2", label: "3:2" },
          { value: "2:3", label: "2:3" },
          { value: "3:4", label: "3:4" },
          { value: "4:3", label: "4:3" },
          { value: "4:5", label: "4:5" },
          { value: "5:4", label: "5:4" },
          { value: "9:16", label: "9:16" },
          { value: "16:9", label: "16:9" },
          { value: "21:9", label: "21:9" },
          { value: "1:4", label: "1:4" },
          { value: "1:8", label: "1:8" },
          { value: "8:1", label: "8:1" },
        ],
      },
    ],
  },
  {
    label: "Nano Banana Pro",
    value: "gemini-3-pro-image",
    description:
      "Powered by Gemini 3 Pro, this premium model delivers high-fidelity image generation with resolution options up to 2K. Combines flexible aspect ratios with superior detail and clarity.",
    modalities: ["image"],
    features: ["generate"],
    settings: [
      {
        name: "Resolution",
        key: "resolution",
        icon: MdPhotoSizeSelectLarge,
        tooltip: "Resolution of the image",
        defaultValue: "1K",
        values: [
          { value: "1K", label: "1K" },
          { value: "2K", label: "2K" },
        ],
      },
      {
        name: "Aspect Ratio",
        key: "aspect_ratio",
        icon: MdCropSquare,
        tooltip: "Aspect ratio of image",
        defaultValue: "1:1",
        values: [
          { value: "1:1", label: "1:1" },
          { value: "3:2", label: "3:2" },
          { value: "2:3", label: "2:3" },
          { value: "3:4", label: "3:4" },
          { value: "4:3", label: "4:3" },
          { value: "4:5", label: "4:5" },
          { value: "5:4", label: "5:4" },
          { value: "9:16", label: "9:16" },
          { value: "16:9", label: "16:9" },
          { value: "21:9", label: "21:9" },
        ],
      },
    ],
  },
] as const;

export function useModels() {
  const [models, setModels] = useState<Model[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const loadModels = useCallback(async (modality?: string) => {
    setIsLoading(true);
    try {
      await fakeDelay(2000);

      if (modality) {
        const filtered = currentModels.filter((m) =>
          m.modalities.some((mod) => mod === modality),
        );
        setModels(filtered);
      } else {
        setModels(currentModels);
      }
    } finally {
      setIsLoading(false);
    }
  }, []);

  return {
    models,
    isLoading,
    loadModels,
  };
}
