"use client";

import { useIsAuthenticated } from "@azure/msal-react";
import Image from "next/image";
import Link from "next/link";
import LoginButton from "../login-button";

export default function Header() {
  const isAuthenticated = useIsAuthenticated();

  return (
    <header className="flex justify-between mx-auto max-w-[1024px] p-4  border-b">
      <div className="flex items-center">
        <Image src="/lerpz.svg" alt="Lerpz Logo" width={32} height={32} />
        <h1 className="ml-4 text-2xl font-bold">Lerpz AI</h1>
      </div>
      <div>
        <Link href="/docs">Documentation</Link>
      </div>
      <div>
        <LoginButton>{isAuthenticated ? "Switch" : "Login"}</LoginButton>
      </div>
    </header>
  );
}
