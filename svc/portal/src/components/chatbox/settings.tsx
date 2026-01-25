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
import { Brain, LoaderPinwheel, ScanEye } from "lucide-react";
import { useEffect } from "react";
import { type ModelSetting, useModels } from "@/hooks/useModels";
import { useChatboxStore } from "@/store/chatbox.store";
import { useChatbox } from "./provider";

type ModelSelectValue = string | null;

interface ModelSelectItem {
  label: string;
  value: ModelSelectValue;
}

export default function ChatboxSettings() {
  const { variant } = useChatbox();
  const { model, setModel } = useChatboxStore();
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
      <Select
        items={modelsWithNone}
        value={model ?? null}
        onValueChange={(model) => setModel(model || undefined)}
      >
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
      {(() => {
        if (!model) return null;
        const found = models.find((m) => m.value === model);
        if (!found) return null;

        return <ChatboxSettingsImage settings={found.settings} />;
      })()}
    </div>
  );
}

interface ChatboxSettingsImageProps {
  settings: ModelSetting[];
}

function ChatboxSettingsImage({ settings }: ChatboxSettingsImageProps) {
  const { autoAnalyze, setAutoAnalyze } = useChatboxStore();

  const toggleAutoAnalyze = () => {
    setAutoAnalyze(!autoAnalyze);
  };

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

      {settings.map((setting) => (
        <Select
          key={setting.name}
          items={setting.values}
          value={setting.values[0]}
          // onValueChange={setter as (v: string | null) => void}
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
      ))}
    </div>
  );
}
