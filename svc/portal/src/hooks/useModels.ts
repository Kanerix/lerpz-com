import { FileType, Images, ImageUpscale, type LucideIcon } from "lucide-react";
import { useCallback, useState } from "react";

export type Model = {
  label: string;
  value: string | null;
  modalities: string[];
  settings: ModelSetting[];
};

export type ModelSetting = {
  key: string;
  name: string;
  icon: LucideIcon;
  tooltip: string;
  values: {
    value: string;
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
    modalities: ["image"],
    settings: [
      {
        name: "Quality",
        key: "quality",
        icon: ImageUpscale,
        tooltip: "Quality of the image(s)",
        values: [
          { value: "auto", label: "Auto" },
          { value: "low", label: "Low" },
          { value: "medium", label: "Medium" },
          { value: "high", label: "High" },
        ],
      },
      {
        name: "Amount",
        key: "amount",
        icon: Images,
        tooltip: "Amount of image(s)",
        values: [
          { value: "1", label: "1" },
          { value: "2", label: "2" },
          { value: "3", label: "3" },
          { value: "4", label: "4" },
          { value: "5", label: "5" },
          { value: "6", label: "6" },
        ],
      },
      {
        name: "Filetype",
        key: "filetype",
        icon: FileType,
        tooltip: "Type of image(s)",
        values: [
          { value: "png", label: "PNG" },
          { value: "webp", label: "WebP" },
          { value: "jpg", label: "JPG" },
        ],
      },
      {
        name: "Background",
        key: "background",
        icon: FileType,
        tooltip: "Background of the image(s)",
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
    modalities: ["image"],
    settings: [
      {
        name: "Quality",
        key: "quality",
        icon: ImageUpscale,
        tooltip: "Quality of the image(s)",
        values: [
          { value: "auto", label: "Auto" },
          { value: "low", label: "Low" },
          { value: "medium", label: "Medium" },
          { value: "high", label: "High" },
        ],
      },
      {
        name: "Amount",
        key: "amount",
        icon: Images,
        tooltip: "Amount of image(s)",
        values: [
          { value: "1", label: "1" },
          { value: "2", label: "2" },
          { value: "3", label: "3" },
          { value: "4", label: "4" },
          { value: "5", label: "5" },
          { value: "6", label: "6" },
        ],
      },
      {
        name: "Filetype",
        key: "filetype",
        icon: FileType,
        tooltip: "Type of image(s)",
        values: [
          { value: "png", label: "PNG" },
          { value: "webp", label: "WebP" },
          { value: "jpg", label: "JPG" },
        ],
      },
      {
        name: "Background",
        key: "background",
        icon: FileType,
        tooltip: "Background of the image(s)",
        values: [
          { value: "auto", label: "Auto" },
          { value: "transparent", label: "Transparent" },
          { value: "opaque", label: "Opaque" },
        ],
      },
    ],
  },
  {
    label: "GPT Image 1 mini",
    value: "gpt-image-1-mini",
    modalities: ["image"],
    settings: [
      {
        name: "Quality",
        key: "quality",
        icon: ImageUpscale,
        tooltip: "Quality of the image(s)",
        values: [
          { value: "auto", label: "Auto" },
          { value: "low", label: "Low" },
          { value: "medium", label: "Medium" },
          { value: "high", label: "High" },
        ],
      },
      {
        name: "Amount",
        key: "amount",
        icon: Images,
        tooltip: "Amount of image(s)",
        values: [
          { value: "1", label: "1" },
          { value: "2", label: "2" },
          { value: "3", label: "3" },
          { value: "4", label: "4" },
          { value: "5", label: "5" },
          { value: "6", label: "6" },
        ],
      },
      {
        name: "Filetype",
        key: "filetype",
        icon: FileType,
        tooltip: "Type of image(s)",
        values: [
          { value: "png", label: "PNG" },
          { value: "webp", label: "WebP" },
          { value: "jpg", label: "JPG" },
        ],
      },
      {
        name: "Background",
        key: "background",
        icon: FileType,
        tooltip: "Background of the image(s)",
        values: [
          { value: "auto", label: "Auto" },
          { value: "transparent", label: "Transparent" },
          { value: "opaque", label: "Opaque" },
        ],
      },
    ],
  },
  {
    label: "Gemini 2.5 Flash image",
    value: "gemini-2.5-flash-image",
    modalities: ["image"],
    settings: [
      {
        name: "Resolution",
        key: "resolution",
        icon: ImageUpscale,
        tooltip: "Quality of the image(s)",
        values: [
          { value: "1K", label: "1K" },
          { value: "2K", label: "2K" },
          { value: "4K", label: "4K" },
        ],
      },
      {
        name: "Filetype",
        key: "filetype",
        icon: FileType,
        tooltip: "Type of image(s)",
        values: [{ value: "png", label: "PNG" }],
      },
      {
        name: "Background",
        key: "background",
        icon: FileType,
        tooltip: "Background of the image(s)",
        values: [
          { value: "auto", label: "Auto" },
          { value: "transparent", label: "Transparent" },
          { value: "opaque", label: "Opaque" },
        ],
      },
    ],
  },
  {
    label: "Gemini 3 Pro image",
    value: "gemini-3-pro-image",
    modalities: ["image"],
    settings: [
      {
        name: "Resolution",
        key: "resolution",
        icon: ImageUpscale,
        tooltip: "Quality of the image(s)",
        values: [
          { value: "1K", label: "1K" },
          { value: "2K", label: "2K" },
          { value: "4K", label: "4K" },
        ],
      },
      {
        name: "Filetype",
        key: "filetype",
        icon: FileType,
        tooltip: "Type of image(s)",
        values: [{ value: "png", label: "PNG" }],
      },
      {
        name: "Background",
        key: "background",
        icon: FileType,
        tooltip: "Background of the image(s)",
        values: [
          { value: "auto", label: "Auto" },
          { value: "transparent", label: "Transparent" },
          { value: "opaque", label: "Opaque" },
        ],
      },
    ],
  },
];

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
