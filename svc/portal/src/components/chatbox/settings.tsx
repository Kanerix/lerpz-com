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
import { Brain, LoaderPinwheel } from "lucide-react";
import { useEffect } from "react";
import { useModels } from "@/hooks/useModels";
import { useChatbox } from "./provider";
import ChatboxSettingsChat from "./settings-chat";
import ChatboxSettingsImage from "./settings-image";
import ChatboxSettingsVideo from "./settings-video";

type ModelSelectValue = string | null;

interface ModelSelectItem {
  label: string;
  value: ModelSelectValue;
}

export default function ChatboxSettings() {
  const { variant } = useChatbox();
  const { models, isLoading: isLoadingModels, loadModels } = useModels();

  const modelItems: ModelSelectItem[] =
    models?.map((m) => ({
      label: m.label,
      value: m.value,
    })) ?? [];

  const modelsWithNone: ModelSelectItem[] = [
    { label: "Default model", value: null },
    ...modelItems,
  ];

  useEffect(() => {
    loadModels(variant);
  }, [variant]);

  return (
    <div className="flex gap-x-4 justify-between mt-4">
      <Select items={modelsWithNone}>
        <Tooltip>
          <TooltipTrigger
            render={
              <SelectTrigger className="relative" disabled={isLoadingModels}>
                <div className="flex items-center">
                  {isLoadingModels ? (
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
            {modelsWithNone.map((model) => (
              <SelectItem key={model.value} value={model.value}>
                {model.label}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>
      {variant === "chat" && <ChatboxSettingsChat />}
      {variant === "image" && <ChatboxSettingsImage />}
      {variant === "video" && <ChatboxSettingsVideo />}
    </div>
  );
}
