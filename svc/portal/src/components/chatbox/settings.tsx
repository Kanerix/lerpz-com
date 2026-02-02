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
import { DEFAULT_IMAGE_MODEL, useChatbox } from "./provider";

type ModelSelectValue = string | null;

interface ModelSelectItem {
  label: string;
  value: ModelSelectValue;
}

export default function ChatboxSettings() {
  const { mode: variant } = useChatbox();
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
    void loadModels(variant);
  }, [variant, loadModels]);

  return (
    <div className="flex gap-x-4 justify-between mt-4">
      <Select items={modelsWithNone} value={model} onValueChange={setModel}>
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
            {modelsWithNone.map((m) => (
              <SelectItem key={m.value ?? "default"} value={m.value}>
                {m.label}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>

      {(() => {
        const selectedModel = model || DEFAULT_IMAGE_MODEL;
        const found = models.find((m) => m.value === selectedModel);
        if (!found) return null;

        return (
          <ChatboxSettingsDynamic
            settings={found.settings}
            modelId={selectedModel}
          />
        );
      })()}
    </div>
  );
}

interface ChatboxSettingsDynamicProps {
  settings: ModelSetting[];
  modelId: string;
}

function ChatboxSettingsDynamic({
  settings,
  modelId,
}: ChatboxSettingsDynamicProps) {
  const { autoAnalyze, setAutoAnalyze, getModelSetting, setModelSetting } =
    useChatboxStore();

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
              aria-label="Auto analyze image(s)"
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
        const currentValue =
          getModelSetting(modelId, setting.name) ?? setting.values[0]?.value;

        return (
          <Select
            key={setting.name}
            items={setting.values}
            value={currentValue}
            onValueChange={(value) =>
              setModelSetting(modelId, setting.name, value)
            }
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
