"use client";

import { useEffect } from "react";
import { useChatbox } from "@/components/chatbox/provider";

export default function Layout({ children }: { children: React.ReactNode }) {
    const { setMode } = useChatbox();

    useEffect(() => {
        setMode("image");
    }, [setMode]);

    return children;
}
