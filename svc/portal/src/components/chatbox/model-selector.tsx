"use client";

import { Button } from "@lerpz/ui/components/button";
import {
    Popover,
    PopoverBackdrop,
    PopoverClose,
    PopoverContent,
    PopoverPortal,
    PopoverPositioner,
    PopoverTrigger,
} from "@lerpz/ui/components/popover";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
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
import { cn } from "@lerpz/ui/lib/utils";
import { MdCheck, MdClose, MdLoop, MdSmartToy } from "react-icons/md";
import type { Model, ModelSetting } from "@/hooks/useModels";
import { useChatbox } from "./provider";
import { useChatboxStore } from "./store";

export function ModelSelectorTrigger() {
    const { models, isModelsLoading } = useChatbox();
    const { model: selectedModel, chatboxAnchor } = useChatboxStore();

    const selected = models.find((m) => m.value === selectedModel);

    return (
        <Popover>
            <PopoverTrigger
                render={
                    <Button
                        variant="outline"
                        disabled={isModelsLoading}
                        className="gap-1.5"
                    >
                        {isModelsLoading ? (
                            <MdLoop className="animate-spin" />
                        ) : (
                            <MdSmartToy />
                        )}
                        <span className="text-sm truncate max-w-[120px]">
                            {selected ? selected.label : "Select model"}
                        </span>
                    </Button>
                }
            />
            <PopoverBackdrop />
            <PopoverPortal>
                <PopoverPositioner
                    side="top"
                    sideOffset={12}
                    align="center"
                    anchor={chatboxAnchor}
                >
                    <PopoverContent className="w-[600px] h-[500px] flex overflow-hidden p-0">
                        <ModelSelectorPanel />
                    </PopoverContent>
                </PopoverPositioner>
            </PopoverPortal>
        </Popover>
    );
}

function ModelSelectorPanel() {
    const { models, isModelsLoading } = useChatbox();
    const { model: selectedModel } = useChatboxStore();

    const selectedModelData = models.find((m) => m.value === selectedModel);

    return (
        <>
            {/* Left column — model list */}
            <div className="w-[220px] shrink-0 border-r flex flex-col overflow-hidden">
                <div className="px-4 py-3 border-b">
                    <h3 className="text-sm font-medium">Models</h3>
                </div>

                {isModelsLoading ? (
                    <div className="flex-1 flex items-center justify-center">
                        <div className="flex flex-col items-center gap-2">
                            <MdLoop className="size-5 animate-spin text-muted-foreground" />
                            <p className="text-xs text-muted-foreground">
                                Loading...
                            </p>
                        </div>
                    </div>
                ) : models.length === 0 ? (
                    <div className="flex-1 flex items-center justify-center">
                        <p className="text-xs text-muted-foreground">
                            No models available
                        </p>
                    </div>
                ) : (
                    <ScrollArea className="flex-1 overflow-hidden">
                        <div className="p-1.5 flex flex-col gap-0.5">
                            {models.map((m) => (
                                <ModelListItem
                                    key={m.value ?? "default"}
                                    model={m}
                                    isSelected={selectedModel === m.value}
                                />
                            ))}
                        </div>
                    </ScrollArea>
                )}
            </div>

            {/* Right column — details + settings */}
            <div className="flex-1 flex flex-col overflow-hidden">
                <div className="flex items-center justify-end px-2 py-1.5 border-b">
                    <PopoverClose
                        render={
                            <Button
                                variant="ghost"
                                size="icon-xs"
                                aria-label="Close model selector"
                            >
                                <MdClose />
                            </Button>
                        }
                    />
                </div>
                {selectedModelData ? (
                    <ModelDetail model={selectedModelData} />
                ) : (
                    <div className="flex-1 flex items-center justify-center px-6">
                        <p className="text-sm text-muted-foreground text-center">
                            Select a model to view its details and settings.
                        </p>
                    </div>
                )}
            </div>
        </>
    );
}

function ModelListItem({
    model,
    isSelected,
}: {
    model: Model;
    isSelected: boolean;
}) {
    const { setModel } = useChatboxStore();

    return (
        <button
            type="button"
            onClick={() => setModel(model.value)}
            className={cn(
                "w-full rounded-lg px-3 py-2 text-left transition-colors",
                "hover:bg-accent hover:text-accent-foreground",
                "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
                isSelected && "bg-accent text-accent-foreground",
            )}
        >
            <div className="flex items-center gap-2">
                <MdSmartToy className="size-3.5 shrink-0 text-muted-foreground" />
                <span className="text-sm font-medium truncate">
                    {model.label}
                </span>
                {isSelected && (
                    <MdCheck className="size-3.5 shrink-0 text-primary ml-auto" />
                )}
            </div>
        </button>
    );
}

function ModelDetail({ model }: { model: Model }) {
    const { getModelSetting, setModelSetting } = useChatboxStore();

    return (
        <ScrollArea className="flex-1 overflow-hidden">
            <div className="p-4 flex flex-col gap-5">
                {/* Header */}
                <div>
                    <h3 className="text-base font-semibold">{model.label}</h3>
                    {model.description && (
                        <p className="mt-1.5 text-sm text-muted-foreground leading-relaxed">
                            {model.description}
                        </p>
                    )}
                </div>

                {/* Features / modalities */}
                <div className="flex flex-wrap gap-1.5">
                    {model.modalities.map((mod) => (
                        <span
                            key={mod}
                            className="rounded-full bg-primary/10 px-2.5 py-0.5 text-xs font-medium text-primary"
                        >
                            {mod}
                        </span>
                    ))}
                    {model.features.map((feat) => (
                        <span
                            key={feat}
                            className="rounded-full bg-muted px-2.5 py-0.5 text-xs font-medium text-muted-foreground"
                        >
                            {feat}
                        </span>
                    ))}
                </div>

                {/* Settings */}
                {model.settings.length > 0 && (
                    <div className="flex flex-col gap-3">
                        <h4 className="text-sm font-medium text-muted-foreground">
                            Settings
                        </h4>
                        <div className="flex flex-col gap-3">
                            {model.settings.map((setting) => (
                                <ModelSettingRow
                                    key={setting.key}
                                    setting={setting}
                                    currentValue={
                                        getModelSetting(
                                            model.value ?? undefined,
                                            setting.key,
                                        ) ??
                                        setting.defaultValue ??
                                        null
                                    }
                                    onChange={(value) => {
                                        if (model.value) {
                                            setModelSetting(
                                                model.value,
                                                setting.key,
                                                value,
                                            );
                                        }
                                    }}
                                />
                            ))}
                        </div>
                    </div>
                )}
            </div>
        </ScrollArea>
    );
}

function ModelSettingRow({
    setting,
    currentValue,
    onChange,
}: {
    setting: ModelSetting;
    currentValue: string | null;
    onChange: (value: string | null) => void;
}) {
    const Icon = setting.icon;

    const items = setting.values.map((v) => ({
        label: v.label,
        value: String(v.value),
    }));

    return (
        <div className="flex items-center justify-between gap-3">
            <Tooltip>
                <TooltipTrigger
                    render={
                        <div className="flex items-center gap-2 min-w-0">
                            <Icon className="size-3.5 shrink-0 text-muted-foreground" />
                            <span className="text-sm truncate">
                                {setting.name}
                            </span>
                        </div>
                    }
                />
                <TooltipContent>
                    <p>{setting.tooltip}</p>
                </TooltipContent>
            </Tooltip>

            <Select items={items} value={currentValue} onValueChange={onChange}>
                <SelectTrigger size="sm" className="w-[120px] shrink-0">
                    <SelectValue placeholder="Default" />
                </SelectTrigger>
                <SelectContent className="w-fit" alignItemWithTrigger={false}>
                    <SelectGroup>
                        {items.map((item) => (
                            <SelectItem key={item.value} value={item.value}>
                                {item.label}
                            </SelectItem>
                        ))}
                    </SelectGroup>
                </SelectContent>
            </Select>
        </div>
    );
}
