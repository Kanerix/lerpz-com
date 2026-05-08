import { useCallback, useState } from "react";

export type ChatboxVariant = "chat" | "image" | "video";

const fakeDelay = (ms: number) =>
    new Promise<void>((resolve) => setTimeout(resolve, ms));

export function useEnhance() {
    const [isLoading, setIsLoading] = useState(false);

    const enhance = useCallback(async (_prompt: string) => {
        setIsLoading(true);
        try {
            await fakeDelay(2000);

            return "Generate a list of 10 AI models with their labels, values, and variants (chat, image, video) in JSON format.";
        } finally {
            setIsLoading(false);
        }
    }, []);

    return {
        isLoading,
        enhance,
    };
}
