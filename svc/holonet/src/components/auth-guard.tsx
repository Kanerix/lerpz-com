"use client";

import { InteractionStatus } from "@azure/msal-browser";
import { useMsal } from "@azure/msal-react";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { LoadingPage } from "@/components/loading-page";

export function AuthGuard({ children }: { children: React.ReactNode }) {
    const { inProgress, accounts } = useMsal();
    const router = useRouter();

    useEffect(() => {
        if (inProgress === InteractionStatus.None && accounts.length === 0) {
            router.push("/login");
        }
    }, [inProgress, accounts, router]);

    if (inProgress !== InteractionStatus.None) {
        return <LoadingPage />;
    }

    if (accounts.length === 0) {
        return <LoadingPage message="Redirecting to sign in..." />;
    }

    return children;
}
