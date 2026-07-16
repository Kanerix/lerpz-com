<script lang="ts">
import { InteractionStatus } from "@azure/msal-browser";
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@lerpz/ui/components/card";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { msalStore } from "$lib/auth/msal.svelte.js";
import ThemeButton from "$lib/components/ThemeButton.svelte";

const error = $derived(page.url.searchParams.get("error"));
const errorDescription = $derived(
    page.url.searchParams.get("error_description"),
);

// `page.url.searchParams.get` already decodes percent-encoding, but error
// descriptions coming from an OAuth redirect may still arrive with `+` for
// spaces and an extra layer of encoding. Decode defensively so a malformed
// value (e.g. a literal `%`) can never throw during render.
function safeDecode(value: string): string {
    try {
        return decodeURIComponent(value.replace(/\+/g, " "));
    } catch {
        return value.replace(/\+/g, " ");
    }
}

const description = $derived(
    errorDescription ? safeDecode(errorDescription) : null,
);

$effect(() => {
    if (
        msalStore.inProgress === InteractionStatus.None &&
        msalStore.accounts.length > 0 &&
        !error
    ) {
        goto("/ai/chats", { replaceState: true });
    }
});

async function handleLogin() {
    try {
        await msalStore.loginRedirect();
    } catch {
        console.error("Login failed");
    }
}

function dismissError() {
    goto("/login", { replaceState: true });
}
</script>

<div class="flex min-h-screen flex-col items-center justify-center bg-background px-4">
  <div class="absolute top-4 right-4">
    <ThemeButton />
  </div>

  <div class="w-full max-w-sm">
    <div class="mb-8 flex flex-col items-center gap-2">
      <img src="/lerpz.svg" alt="Lerpz" class="h-12 w-12" />
      <h1 class="text-2xl font-semibold tracking-tight">Lerpz AI</h1>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>Sign in</CardTitle>
        <CardDescription>Sign in with your organization account to continue.</CardDescription>
      </CardHeader>

      <CardContent class="flex flex-col gap-4">
        {#if error}
          <div
            role="alert"
            class="flex flex-col gap-3 rounded-lg border border-destructive/50 bg-destructive/10 px-4 py-3 text-sm text-destructive"
          >
            <div class="flex items-start gap-2">
              <Icon icon="fa6-solid:circle-exclamation" class="mt-0.5 size-4 shrink-0" />
              <div class="min-w-0 flex-1">
                <p class="font-medium">Authentication failed</p>
                <p class="mt-1 break-all text-destructive/80">
                  {description ?? "Something went wrong while signing you in."}
                </p>
                <p class="mt-2 font-mono text-xs text-destructive/60">{error}</p>
              </div>
            </div>
            <Button
              variant="outline"
              size="sm"
              onclick={dismissError}
              class="w-full border-destructive/40 bg-transparent text-xs text-destructive hover:bg-destructive/10 hover:text-destructive"
            >
              Dismiss
            </Button>
          </div>
        {/if}

        <Button
          size="lg"
          onclick={handleLogin}
          disabled={msalStore.inProgress !== InteractionStatus.None}
          class="w-full gap-2"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 23 23" class="size-4" aria-hidden="true">
            <path fill="#f35325" d="M1 1h10v10H1z" />
            <path fill="#81bc06" d="M12 1h10v10H12z" />
            <path fill="#05a6f0" d="M1 12h10v10H1z" />
            <path fill="#ffba08" d="M12 12h10v10H12z" />
          </svg>
          Sign in with Microsoft
        </Button>
      </CardContent>

      <CardFooter>
        <p class="w-full text-center text-xs text-muted-foreground">
          By signing in, you agree to your organization's policies.
        </p>
      </CardFooter>
    </Card>
  </div>
</div>
