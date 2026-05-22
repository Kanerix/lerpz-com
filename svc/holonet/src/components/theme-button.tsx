"use client";

import { Button } from "@lerpz/ui/components/button";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { useTheme } from "next-themes";
import { MdDarkMode, MdLightMode } from "react-icons/md";

export function ThemeButton() {
    const { setTheme, theme } = useTheme();

    return (
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger
                    render={
                        <Button
                            variant="ghost"
                            size="icon"
                            onClick={() =>
                                setTheme(theme === "dark" ? "light" : "dark")
                            }
                        >
                            <MdLightMode className="h-5 w-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
                            <MdDarkMode className="absolute h-5 w-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
                            <span className="sr-only">Toggle theme</span>
                        </Button>
                    }
                />
                <TooltipContent>
                    <p>Toggle theme</p>
                </TooltipContent>
            </Tooltip>
        </TooltipProvider>
    );
}
