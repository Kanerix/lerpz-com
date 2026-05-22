"use client";

import { useMsal } from "@azure/msal-react";
import { Button } from "@lerpz/ui/components/button";

export function LogoutButton() {
    const { instance } = useMsal();

    const handleLogout = () => {
        instance.logoutRedirect().catch((error) => {
            console.error("MSAL logout error:", error);
        });
    };

    return <Button onClick={handleLogout}>Sign out</Button>;
}
