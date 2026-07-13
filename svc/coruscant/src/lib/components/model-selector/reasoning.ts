// Reasoning levels are a model-level concept shared by any surface that renders
// a model picker (chat, image, …), so they live alongside the ModelSelector
// rather than inside a single feature's store.

export const REASONING_KEY = "reasoning";

export const REASONING_LEVELS = [
    { value: "none", label: "None" },
    { value: "low", label: "Low" },
    { value: "medium", label: "Medium" },
    { value: "high", label: "High" },
] as const;

export const DEFAULT_REASONING_LEVEL = "medium";
