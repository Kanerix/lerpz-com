import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@lerpz/ui/components/select";
import { Toggle } from "@lerpz/ui/components/toggle";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { FileType, Images, ImageUpscale, ScanEye } from "lucide-react";
import { useChatboxImageStore } from "@/store/chatbox.store";

const settings = [
  {
    name: "Quality",
    key: "quality" as const,
    icon: ImageUpscale,
    tooltip: "Quality of the image(s)",
    values: [
      { value: "1K", label: "1K" },
      { value: "2K", label: "2K" },
      { value: "4K", label: "4K" },
    ],
  },
  {
    name: "Amount",
    key: "amount" as const,
    icon: Images,
    tooltip: "Amount of image(s) to generate",
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
    key: "filetype" as const,
    icon: FileType,
    tooltip: "Type of image(s) to create",
    values: [
      { value: "png", label: "PNG" },
      { value: "webp", label: "WebP" },
      { value: "jpg", label: "JPG" },
    ],
  },
  {
    name: "Background",
    key: "background" as const,
    icon: FileType,
    tooltip: "Background of the image(s)",
    values: [
      { value: "auto", label: "Auto" },
      { value: "transparent", label: "Transparent" },
      { value: "opaque", label: "Opaque" },
    ],
  },
];

export default function ChatboxSettingsImage() {
  const {
    autoAnalyze,
    toggleAutoAnalyze,
    quality,
    amount,
    filetype,
    background,
    setQuality,
    setAmount,
    setFiletype,
    setBackground,
  } = useChatboxImageStore();

  const currentValues = { quality, amount, filetype, background };

  return (
    <div className="flex gap-x-4 justify-between">
      <Tooltip>
        <TooltipTrigger
          render={
            <Toggle
              pressed={autoAnalyze}
              onPressedChange={toggleAutoAnalyze}
              variant="outline"
              aria-label="Auto analyze image(s) generated"
            >
              <ScanEye />
            </Toggle>
          }
        />
        <TooltipContent>
          <p>Analyze created image(s)</p>
        </TooltipContent>
      </Tooltip>

      {settings.map((setting) => {
        const value = currentValues[setting.key];

        // TODO: This is ugly, fix in future.
        const setter =
          setting.key === "quality"
            ? setQuality
            : setting.key === "amount"
              ? setAmount
              : setting.key === "filetype"
                ? setFiletype
                : setBackground;

        return (
          <Select
            key={setting.name}
            items={setting.values}
            value={value}
            onValueChange={setter as (v: string | null) => void}
          >
            <Tooltip>
              <TooltipTrigger
                render={
                  <SelectTrigger className="relative">
                    <div className="flex items-center">
                      <setting.icon className="mr-2" />
                      <SelectValue />
                    </div>
                  </SelectTrigger>
                }
              />
              <TooltipContent>
                <p>{setting.tooltip}</p>
              </TooltipContent>
            </Tooltip>
            <SelectContent className="w-fit" alignItemWithTrigger={false}>
              <SelectGroup>
                <SelectLabel>{setting.name}</SelectLabel>
                {setting.values.map((item) => (
                  <SelectItem key={item.value} value={item.value}>
                    {item.label}
                  </SelectItem>
                ))}
              </SelectGroup>
            </SelectContent>
          </Select>
        );
      })}
    </div>
  );
}
