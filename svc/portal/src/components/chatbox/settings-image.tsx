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
import { useModels } from "@/hooks/useModels";

type ModelSelectValue = string | null;

interface ModelSelectItem {
  label: string;
  value: ModelSelectValue;
}

const MODEL_VARIANT = "image";

export default function ChatboxSettingsImage() {
  const { models, isLoading: isLoadingModels } = useModels({
    variant: MODEL_VARIANT,
  });

  const modelItems: ModelSelectItem[] =
    models?.map((m) => ({
      label: m.label,
      value: m.value,
    })) ?? [];

  const modelsWithNone: ModelSelectItem[] = [
    { label: "Select model", value: null },
    ...modelItems,
  ];

  console.log("COMP", isLoadingModels)

  return (
    <div className="gap-x-4 justify-between mt-6">
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
    </div>
  );
}
