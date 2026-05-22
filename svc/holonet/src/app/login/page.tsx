"use client";

import { InteractionStatus } from "@azure/msal-browser";
import { useMsal } from "@azure/msal-react";
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@lerpz/ui/components/card";
import Image from "next/image";
import { useRouter, useSearchParams } from "next/navigation";
import { useEffect } from "react";
import { LoadingPage } from "@/components/loading-page";
import LoginButton from "@/components/login-button";
import { ThemeButton } from "@/components/theme-button";

export default function LoginPage() {
    const { inProgress, accounts } = useMsal();
    const router = useRouter();
    const searchParams = useSearchParams();

    const error = searchParams.get("error");
    const errorDescription = searchParams.get("error_description");

    useEffect(() => {
        if (
            inProgress === InteractionStatus.None &&
            accounts.length > 0 &&
            !error
        ) {
            router.replace("/ai/chats");
        }
    }, [inProgress, accounts, router, error]);

    if (inProgress !== InteractionStatus.None) {
        return <LoadingPage />;
    }

    return (
        <div className="flex min-h-screen flex-col items-center justify-center bg-background px-4">
            <div className="absolute top-4 right-4">
                <ThemeButton />
            </div>

            <div className="w-full max-w-sm">
                <div className="mb-8 flex flex-col items-center gap-2">
                    <Image
                        src="/lerpz.svg"
                        alt="Lerpz"
                        width={48}
                        height={48}
                        className="h-12 w-12"
                        priority
                    />
                    <h1 className="text-2xl font-semibold tracking-tight">
                        Lerpz AI
                    </h1>
                </div>

                <Card>
                    <CardHeader>
                        <CardTitle>Sign in</CardTitle>
                        <CardDescription>
                            Sign in with your organization account to continue.
                        </CardDescription>
                    </CardHeader>

                    <CardContent className="flex flex-col gap-4">
                        {error && (
                            <div className="rounded-lg border border-destructive/50 bg-destructive/10 px-4 py-3 text-sm text-destructive">
                                <p className="font-medium">
                                    Authentication failed
                                </p>
                                <p className="mt-1 text-destructive/80">
                                    {errorDescription
                                        ? decodeURIComponent(
                                              errorDescription.replace(
                                                  /\+/g,
                                                  " ",
                                              ),
                                          )
                                        : error}
                                </p>
                            </div>
                        )}

                        <LoginButton className="w-full h-10">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 23 23"
                                className="size-4"
                                aria-hidden="true"
                            >
                                <path fill="#f35325" d="M1 1h10v10H1z" />
                                <path fill="#81bc06" d="M12 1h10v10H12z" />
                                <path fill="#05a6f0" d="M1 12h10v10H1z" />
                                <path fill="#ffba08" d="M12 12h10v10H12z" />
                            </svg>
                            Sign in with Microsoft
                        </LoginButton>
                    </CardContent>

                    <CardFooter>
                        <p className="w-full text-center text-xs text-muted-foreground">
                            By signing in, you agree to your organization&apos;s
                            policies.
                        </p>
                    </CardFooter>
                </Card>
            </div>
        </div>
    );
}
