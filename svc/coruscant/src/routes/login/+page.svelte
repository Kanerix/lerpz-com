<script lang="ts">
import { InteractionStatus } from "@azure/msal-browser";
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@lerpz/ui/components/card";
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import ThemeButton from "$lib/components/ThemeButton.svelte";
import { msalStore } from "$lib/auth/msal.svelte.js";

const error = $derived($page.url.searchParams.get("error"));
const errorDescription = $derived(
    $page.url.searchParams.get("error_description"),
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
          <div class="rounded-lg border border-destructive/50 bg-destructive/10 px-4 py-3 text-sm text-destructive">
            <p class="font-medium">Authentication failed</p>
            <p class="mt-1 text-destructive/80">
              {errorDescription
                ? decodeURIComponent(errorDescription.replace(/\+/g, " "))
                : error}
            </p>
          </div>
        {/if}

        <button
          onclick={handleLogin}
          disabled={msalStore.inProgress !== InteractionStatus.None}
          class="inline-flex h-10 w-full items-center justify-center gap-2 rounded-4xl bg-primary px-4 text-sm font-medium text-primary-foreground transition hover:bg-primary/80 disabled:opacity-50"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 23 23" class="size-4" aria-hidden="true">
            <path fill="#f35325" d="M1 1h10v10H1z" />
            <path fill="#81bc06" d="M12 1h10v10H12z" />
            <path fill="#05a6f0" d="M1 12h10v10H1z" />
            <path fill="#ffba08" d="M12 12h10v10H12z" />
          </svg>
          Sign in with Microsoft
        </button>
      </CardContent>

      <CardFooter>
        <p class="w-full text-center text-xs text-muted-foreground">
          By signing in, you agree to your organization's policies.
        </p>
      </CardFooter>
    </Card>
  </div>
</div>
