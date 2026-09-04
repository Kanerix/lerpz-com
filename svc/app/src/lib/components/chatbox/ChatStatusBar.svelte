<script lang="ts">
import { PromptStatusBar } from "$lib/components/prompt";

let {
    isThinking = false,
    isSaved = false,
    error = null,
    onShowError,
}: {
    isThinking?: boolean;
    isSaved?: boolean;
    error?: string | null;
    /** Called when the user clicks the "Show error" button. */
    onShowError?: () => void;
} = $props();

const quotes = [
    "Thinking really hard…",
    "Consulting the rubber duck…",
    "Reticulating splines…",
    "Summoning electrons…",
    "Untangling the cables…",
    "Asking the magic conch…",
    "Borrowing a cup of RAM…",
    "Counting to infinity…",
    "Polishing the pixels…",
    "Bribing the hamsters…",
    "Aligning the flux capacitor…",
    "Pretending to read the docs…",
];
</script>

<PromptStatusBar
  busy={isThinking}
  saved={isSaved}
  {quotes}
  {error}
  {onShowError}
/>
