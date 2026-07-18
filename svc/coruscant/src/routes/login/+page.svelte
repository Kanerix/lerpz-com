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
import {
    loadLegalConsent,
    storeLegalConsent,
} from "$lib/auth/legal-consent.js";
import { msalStore } from "$lib/auth/msal.svelte.js";
import ThemeButton from "$lib/components/ThemeButton.svelte";

// Whether the user has agreed to the legal policies. Starts `false` so the
// server-rendered markup is stable, then hydrates from the persisted value so a
// returning user is pre-accepted and never has to tick the box again.
let legalAccepted = $state(false);

$effect(() => {
    legalAccepted = loadLegalConsent();
});

function toggleLegalConsent(accepted: boolean) {
    legalAccepted = accepted;
    storeLegalConsent(accepted);
}

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
    if (!legalAccepted) return;
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

<div class="relative flex min-h-screen flex-col items-center justify-center overflow-hidden bg-background px-4">
  <!-- Ambient background glow -->
  <div aria-hidden="true" class="pointer-events-none absolute inset-0 -z-10">
    <div class="absolute -top-40 left-1/2 h-125 w-125 -translate-x-1/2 rounded-full bg-primary/15 blur-3xl"></div>
    <div class="absolute -bottom-40 left-1/2 h-100 w-150 -translate-x-1/2 rounded-full bg-primary/5 blur-3xl"></div>
  </div>

  <div class="absolute top-4 right-4">
    <ThemeButton />
  </div>

  <div class="w-full max-w-sm">
    <div class="mb-8 flex flex-col items-center">
      <div class="flex size-16 items-center justify-center rounded-2xl border border-border/60 bg-card/60 shadow-sm ring-1 ring-primary/5 backdrop-blur-sm">
        <img src="/lerpz.svg" alt="Lerpz" class="size-9" />
      </div>
    </div>

    <Card class="border-border/60 bg-card/70 shadow-xl backdrop-blur-md">
      <CardHeader class="text-center">
        <CardTitle class="text-2xl">Welcome back</CardTitle>
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

        <label class="flex items-start gap-3 rounded-lg border border-border/60 bg-background/40 px-3 py-2.5 text-left text-sm text-muted-foreground transition-colors hover:bg-background/70">
          <input
            type="checkbox"
            checked={legalAccepted}
            onchange={(e) => toggleLegalConsent(e.currentTarget.checked)}
            class="mt-0.5 size-4 shrink-0 rounded border-input accent-primary"
          />
          <span class="leading-relaxed">
            I agree to the
            <a href="/legal/terms" class="font-medium text-foreground underline underline-offset-2 hover:opacity-80">Terms of Service</a>,
            <a href="/legal/privacy" class="font-medium text-foreground underline underline-offset-2 hover:opacity-80">Privacy Policy</a>, and
            <a href="/legal/cookies" class="font-medium text-foreground underline underline-offset-2 hover:opacity-80">Cookie Policy</a>.
          </span>
        </label>

        <Button
          size="lg"
          onclick={handleLogin}
          disabled={msalStore.inProgress !== InteractionStatus.None || !legalAccepted}
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

    <p class="mt-6 text-center text-xs text-muted-foreground">
      Secured by Microsoft Entra ID
    </p>
  </div>
</div>
