"use client";

import { useIsAuthenticated } from "@azure/msal-react";
import Image from "next/image";
import Link from "next/link";
import LoginButton from "../login-button";

const navLinks = [
    { label: "Tools", href: "/ai" },
    { label: "Roadmap", href: "/roadmap" },
    { label: "Docs", href: "/docs" },
    { label: "Changelog", href: "/changelog" },
    { label: "Status", href: "/status" },
];

export default function Header() {
    const isAuthenticated = useIsAuthenticated();

    return (
        <header className="sticky top-0 z-50 border-b border-border/60 bg-background/80 backdrop-blur-md">
            <div className="mx-auto flex max-w-[1024px] items-center justify-between px-4 py-3">
                {/* Logo */}
                <Link
                    href="/"
                    className="flex items-center gap-2.5 font-bold text-foreground transition-opacity hover:opacity-80"
                >
                    <Image
                        src="/lerpz.svg"
                        alt="Lerpz Logo"
                        width={28}
                        height={28}
                    />
                    <span className="text-lg tracking-tight">Lerpz AI</span>
                </Link>

                {/* Nav */}
                <nav className="hidden items-center gap-6 md:flex">
                    {navLinks.map((link) => (
                        <Link
                            key={link.href}
                            href={link.href}
                            className="text-sm font-medium text-muted-foreground transition-colors hover:text-foreground"
                        >
                            {link.label}
                        </Link>
                    ))}
                </nav>

                {/* Actions */}
                <div className="flex items-center gap-3">
                    {isAuthenticated && (
                        <Link
                            href="/ai/chats"
                            className="hidden text-sm font-medium text-muted-foreground transition-colors hover:text-foreground sm:block"
                        >
                            Dashboard
                        </Link>
                    )}
                    <LoginButton>
                        {isAuthenticated ? "Switch account" : "Sign in"}
                    </LoginButton>
                </div>
            </div>
        </header>
    );
}
