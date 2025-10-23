"use client";

import { useMsal } from "@azure/msal-react";
import { Button } from "@lerpz/ui/components/button";

export default function LogoutButton() {
  const { instance } = useMsal();

  const handleLogout = async () => {
    try {
      await instance.logoutRedirect({
        postLogoutRedirectUri: "/login",
      });
    } catch (error) {
      console.error("Logout failed:", error);
    }
  };

  return (
    <Button variant="destructive" onClick={handleLogout}>
      logout
    </Button>
  );
}
