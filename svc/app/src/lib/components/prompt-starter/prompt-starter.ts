export type PromptExample = {
    /** Iconify name shown beside the title, such as `fa6-solid:lightbulb`. */
    icon: string;
    /** Short label for the kind of task, such as "Explain a concept". */
    title: string;
    /** Prompt text loaded into the composer when the card is picked. */
    prompt: string;
};
