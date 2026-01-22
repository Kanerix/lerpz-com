import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@lerpz/ui/components/select";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { Brain } from "lucide-react";

export default function ChatboxSettingsImage() {
  const models = [
    { label: "GPT Image 1", value: "gpt-image-1" },
    { label: "GPT Image 1.5", value: "gpt-image-1.5" },
    { label: "GPT Image 1 mini", value: "gpt-image-1-mini" },
    { label: "Gemini 2.5 Flash image", value: "gemini-2.5-flash-image" },
    { label: "Gemini 3 Pro image", value: "gemini-3-pro-image" },
  ];

  return (
    <div className="gap-x-4 justify-between">
      <Select defaultValue={models[0]}>
        <Tooltip>
          <TooltipTrigger
            render={
              <SelectTrigger className="relative">
                <div className="flex items-center">
                  <Brain className="mr-2" />
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
            {models.map((o) => (
              <SelectItem key={o.value} value={o.value}>
                {o.label}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>
    </div>
  );
}
